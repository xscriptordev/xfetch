#!/usr/bin/env python3
"""
sync_roadmap.py — Sync ROADMAP.md checkboxes with GitHub Issues.

This script parses a ROADMAP.md file with a standardized format and
synchronizes the state of each task with its corresponding GitHub issue.
It can also auto-create issues for tasks that don't have one yet.

Expected ROADMAP.md format:
  ## Phase N · Title <!-- phase:label-name -->
  - [ ] Task description (#123)
  - [x] Completed task (#124)
  - [/] In-progress task (#125)
  - [ ] New task without issue           ← auto-created

States:
  [ ]  → open issue, remove 'in-progress' label
  [/]  → open issue, add 'in-progress' label
  [x]  → close issue, remove 'in-progress' label

Usage:
  python sync_roadmap.py                          # uses ./ROADMAP.md
  python sync_roadmap.py --roadmap path/to.md     # custom path
  python sync_roadmap.py --dry-run                # preview without changes
  python sync_roadmap.py --repo owner/repo        # explicit repo

Requires: gh CLI authenticated (https://cli.github.com)
"""

import argparse
import json
import re
import subprocess
import sys
from dataclasses import dataclass
from enum import Enum
from pathlib import Path


class TaskState(Enum):
    TODO = "todo"
    IN_PROGRESS = "in-progress"
    DONE = "done"


@dataclass
class Task:
    title: str
    issue_number: int | None
    state: TaskState
    phase_label: str
    line_number: int


# ── Parsing ──────────────────────────────────────────────────────────────────

PHASE_RE = re.compile(r"^##\s+.+<!--\s*phase:(\S+)\s*-->")
TASK_WITH_ISSUE_RE = re.compile(r"^-\s+\[([ x/])\]\s+(.+?)\s+\(#(\d+)\)\s*$")
TASK_WITHOUT_ISSUE_RE = re.compile(r"^-\s+\[([ x/])\]\s+(.+?)\s*$")


def parse_roadmap(path: Path) -> list[Task]:
    """Parse ROADMAP.md and extract tasks with their states."""
    tasks: list[Task] = []
    current_phase = ""

    with open(path, encoding="utf-8") as f:
        for line_num, line in enumerate(f, start=1):
            phase_match = PHASE_RE.match(line)
            if phase_match:
                current_phase = phase_match.group(1)
                continue

            # Try matching task with issue number first
            task_match = TASK_WITH_ISSUE_RE.match(line)
            if task_match:
                checkbox, title, issue_str = task_match.groups()
                issue_num = int(issue_str)
            else:
                # Try matching task without issue number
                task_match = TASK_WITHOUT_ISSUE_RE.match(line)
                if task_match and current_phase:
                    checkbox, title = task_match.groups()
                    issue_num = None
                else:
                    continue

            if checkbox == "x":
                state = TaskState.DONE
            elif checkbox == "/":
                state = TaskState.IN_PROGRESS
            else:
                state = TaskState.TODO

            tasks.append(Task(
                title=title,
                issue_number=issue_num,
                state=state,
                phase_label=current_phase,
                line_number=line_num,
            ))

    return tasks


# ── GitHub CLI helpers ───────────────────────────────────────────────────────

def gh(args: list[str], repo: str | None = None) -> str:
    """Run a gh CLI command and return stdout."""
    cmd = ["gh"] + args
    if repo:
        cmd += ["--repo", repo]
    result = subprocess.run(cmd, capture_output=True, text=True, check=True)
    return result.stdout.strip()


def get_issue(number: int, repo: str | None) -> dict:
    """Fetch issue state and labels from GitHub."""
    raw = gh(["issue", "view", str(number), "--json", "state,labels,title"], repo)
    return json.loads(raw)


def ensure_label_exists(label: str, repo: str | None, color: str = "ededed") -> None:
    """Create a label if it doesn't exist."""
    try:
        gh(["label", "create", label, "--color", color,
            "--description", f"Label: {label}"], repo)
    except subprocess.CalledProcessError:
        pass  # label already exists


def find_existing_issue(title: str, repo: str | None) -> int | None:
    """Search for an open or closed issue with the exact same title. Returns its number or None."""
    try:
        raw = gh(["issue", "list", "--search", f"in:title {title}",
                  "--state", "all", "--json", "number,title", "--limit", "10"], repo)
        issues = json.loads(raw)
        for issue in issues:
            if issue["title"].strip() == title.strip():
                return issue["number"]
    except subprocess.CalledProcessError:
        pass
    return None


def create_issue(title: str, label: str, repo: str | None) -> int:
    """Create a new GitHub issue and return its number."""
    url = gh(["issue", "create",
              "--title", title,
              "--body", f"Auto-created from ROADMAP.md\n\nPhase: `{label}`",
              "--label", label], repo)
    # gh issue create prints the URL: https://github.com/owner/repo/issues/42
    match = re.search(r"/issues/(\d+)", url)
    if not match:
        raise RuntimeError(f"Could not parse issue number from gh output: {url}")
    return int(match.group(1))


def close_issue(number: int, repo: str | None) -> None:
    gh(["issue", "close", str(number)], repo)


def reopen_issue(number: int, repo: str | None) -> None:
    gh(["issue", "reopen", str(number)], repo)


def add_label(number: int, label: str, repo: str | None) -> None:
    gh(["issue", "edit", str(number), "--add-label", label], repo)


def remove_label(number: int, label: str, repo: str | None) -> None:
    try:
        gh(["issue", "edit", str(number), "--remove-label", label], repo)
    except subprocess.CalledProcessError:
        pass  # label wasn't on the issue


# ── Roadmap file updater ─────────────────────────────────────────────────────

def update_roadmap_line(path: Path, line_number: int, title: str,
                        checkbox: str, issue_number: int) -> None:
    """Update a specific line in the ROADMAP.md to include the issue number."""
    lines = path.read_text(encoding="utf-8").splitlines(keepends=True)
    idx = line_number - 1
    lines[idx] = f"- [{checkbox}] {title} (#{issue_number})\n"
    path.write_text("".join(lines), encoding="utf-8")


def git_commit_roadmap(path: Path) -> None:
    """Commit the updated ROADMAP.md with auto-generated issue numbers."""
    subprocess.run(
        ["git", "config", "user.name", "roadmap-sync[bot]"],
        capture_output=True, check=False,
    )
    subprocess.run(
        ["git", "config", "user.email", "roadmap-sync[bot]@users.noreply.github.com"],
        capture_output=True, check=False,
    )
    subprocess.run(["git", "add", str(path)], capture_output=True, check=True)
    subprocess.run(
        ["git", "commit", "-m",
         "docs: update ROADMAP.md with issue numbers [roadmap-sync]"],
        capture_output=True, check=True,
    )
    subprocess.run(["git", "push"], capture_output=True, check=True)


# ── Sync logic ───────────────────────────────────────────────────────────────

IN_PROGRESS_LABEL = "in-progress"


def sync_task(task: Task, repo: str | None, dry_run: bool) -> list[str]:
    """Sync a single task that already has an issue. Returns actions taken."""
    actions: list[str] = []
    issue = get_issue(task.issue_number, repo)
    is_open = issue["state"] == "OPEN"
    has_in_progress = any(
        l["name"] == IN_PROGRESS_LABEL for l in issue.get("labels", [])
    )

    if task.state == TaskState.DONE:
        if is_open:
            actions.append(f"  close #{task.issue_number}")
            if not dry_run:
                close_issue(task.issue_number, repo)
        if has_in_progress:
            actions.append(f"  remove '{IN_PROGRESS_LABEL}' from #{task.issue_number}")
            if not dry_run:
                remove_label(task.issue_number, IN_PROGRESS_LABEL, repo)

    elif task.state == TaskState.IN_PROGRESS:
        if not is_open:
            actions.append(f"  reopen #{task.issue_number}")
            if not dry_run:
                reopen_issue(task.issue_number, repo)
        if not has_in_progress:
            actions.append(f"  add '{IN_PROGRESS_LABEL}' to #{task.issue_number}")
            if not dry_run:
                add_label(task.issue_number, IN_PROGRESS_LABEL, repo)

    elif task.state == TaskState.TODO:
        if not is_open:
            actions.append(f"  reopen #{task.issue_number}")
            if not dry_run:
                reopen_issue(task.issue_number, repo)
        if has_in_progress:
            actions.append(f"  remove '{IN_PROGRESS_LABEL}' from #{task.issue_number}")
            if not dry_run:
                remove_label(task.issue_number, IN_PROGRESS_LABEL, repo)

    return actions


def main() -> None:
    parser = argparse.ArgumentParser(
        description="Sync ROADMAP.md checkboxes with GitHub Issues"
    )
    parser.add_argument(
        "--roadmap", default="ROADMAP.md",
        help="Path to the ROADMAP.md file (default: ./ROADMAP.md)"
    )
    parser.add_argument(
        "--repo", default=None,
        help="GitHub repo in owner/name format (default: auto-detect)"
    )
    parser.add_argument(
        "--dry-run", action="store_true",
        help="Preview changes without applying them"
    )
    args = parser.parse_args()

    roadmap_path = Path(args.roadmap)
    if not roadmap_path.exists():
        print(f"Error: {roadmap_path} not found", file=sys.stderr)
        sys.exit(1)

    tasks = parse_roadmap(roadmap_path)
    if not tasks:
        print("No tasks found in roadmap. Check the format.")
        sys.exit(1)

    prefix = "[DRY RUN] " if args.dry_run else ""
    print(f"{prefix}Found {len(tasks)} tasks in roadmap.")
    print()

    # Ensure in-progress label exists
    if not args.dry_run:
        ensure_label_exists(IN_PROGRESS_LABEL, args.repo, "fbca04")

    # ── Phase 1: Auto-create issues for tasks without (#N) ────────────────
    # This includes [x] tasks — they get created and immediately closed
    new_tasks = [t for t in tasks if t.issue_number is None]
    if new_tasks:
        done_count = sum(1 for t in new_tasks if t.state == TaskState.DONE)
        open_count = len(new_tasks) - done_count
        print(f"{prefix}Creating {len(new_tasks)} new issue(s) "
              f"({open_count} open, {done_count} done → create+close)...")
        roadmap_modified = False
        for task in new_tasks:
            checkbox_char = {"todo": " ", "in-progress": "/", "done": "x"}[task.state.value]
            if args.dry_run:
                suffix = " → would create + close" if task.state == TaskState.DONE else ""
                print(f"  would create: \"{task.title}\" [{task.phase_label}]{suffix}")
            else:
                # Check for existing issue with same title to avoid duplicates
                existing = find_existing_issue(task.title, args.repo)
                if existing:
                    print(f"  found existing #{existing}: \"{task.title}\" (skipping creation)")
                    issue_num = existing
                else:
                    ensure_label_exists(task.phase_label, args.repo)
                    issue_num = create_issue(task.title, task.phase_label, args.repo)
                    print(f"  created #{issue_num}: \"{task.title}\"")

                # If the task is already done, close the issue immediately
                if task.state == TaskState.DONE:
                    close_issue(issue_num, args.repo)
                    print(f"  closed #{issue_num} (already done)")

                task.issue_number = issue_num
                update_roadmap_line(
                    roadmap_path, task.line_number,
                    task.title, checkbox_char, issue_num
                )
                roadmap_modified = True

        if roadmap_modified:
            print()
            print("Committing updated ROADMAP.md...")
            try:
                git_commit_roadmap(roadmap_path)
                print("Pushed updated ROADMAP.md with issue numbers.")
            except subprocess.CalledProcessError as e:
                print(f"Warning: could not auto-commit: {e}", file=sys.stderr)
                print("Please commit ROADMAP.md manually.")
        print()

    # ── Phase 2: Sync state for all tasks with issue numbers ──────────────
    tasks_with_issues = [t for t in tasks if t.issue_number is not None]
    print(f"{prefix}Syncing {len(tasks_with_issues)} task(s)...")

    total_actions = 0
    for task in tasks_with_issues:
        actions = sync_task(task, args.repo, args.dry_run)
        if actions:
            print(f"#{task.issue_number} — {task.title} [{task.state.value}]")
            for action in actions:
                print(action)
            total_actions += len(actions)

    print()
    if total_actions == 0 and not new_tasks:
        print("Everything is already in sync.")
    else:
        verb = "Would apply" if args.dry_run else "Applied"
        created = f", created {len(new_tasks)} issue(s)" if new_tasks else ""
        print(f"{verb} {total_actions} action(s){created}.")


if __name__ == "__main__":
    main()
