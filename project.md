# xfetch Project

**xfetch** is a highly customizable, cross-platform system information fetching tool written in Rust. Inspired by tools like `fastfetch` and `neofetch`, it prioritizes performance, flexibility, and extensive styling options. It supports Linux, Windows, and macOS, and allows users to define custom layouts, logos (text and image), icons, and color schemes via a JSONC configuration file.

## Project Goals

The primary goal of xfetch is to provide a unified and visually appealing way to display system information across different operating systems. It aims to be:
*   **Fast**: Minimal execution time.
*   **Flexible**: Users should be able to make it look exactly how they want.
*   **Portable**: Single binary deployment for ease of use.

## Architecture

*   **Language**: Rust (for safety and performance).
*   **System Info**: Uses `sysinfo` crate for gathering hardware and OS details.
*   **Configuration**: Uses `serde` and `json_comments` to parse JSONC config files.
*   **UI/Terminal**: Uses `crossterm` for cross-platform TUI operations and `viuer` for image rendering in supported terminals.

## Changelog

### Completed [X]
- [X] **Core Infrastructure**: Initialized Rust project with necessary dependencies (`sysinfo`, `clap`, `crossterm`, `serde`, `viuer`).
- [X] **Cross-Platform Support**: Implemented detection and fetching logic for Linux, Windows, and macOS.
- [X] **System Information Modules**:
    - [X] OS Name & Architecture
    - [X] Kernel Version
    - [X] Uptime
    - [X] Shell Detection
    - [X] CPU Model & Frequency
    - [X] GPU Detection (Discrete/Integrated)
    - [X] Memory Usage
    - [X] Disk Usage
    - [X] Battery Status
    - [X] Package Count (pacman, dpkg, scoop)
- [X] **Configuration System**:
    - [X] JSONC file support (`config.jsonc`).
    - [X] Custom module ordering.
    - [X] Toggleable ANSI colors.
    - [X] Custom icons per module (Nerd Fonts support).
    - [X] Custom colors per module.
- [X] **Visual Customization**:
    - [X] Custom ASCII art support via text files.
    - [X] Image/SVG logo support via `viuer`.
    - [X] **Pac-Man Layout**: Dedicated layout mode with custom header/footer borders and icons.
    - [X] Side-by-side layout logic (Logo + Info).
- [X] **Assets & Examples**:
    - [X] Created 20+ example configurations (`configs/`).
    - [X] Created sample logos (`logos/` - txt and svg).
- [X] **Documentation**:
    - [X] Installation Guide (`INSTALLATION.md`).
    - [X] Configuration Guide (`CONFIGURATION.md`).
    - [X] Quick Install Script (`install.sh`).

### Future Improvements []
- [ ] **Expanded Package Manager Support**: Add support for `rpm`, `apk`, `nix`, `brew`, `chocolatey`.
- [ ] **Network Module**: Display local/public IP (optional/privacy-focused).
- [ ] **Music Player Info**: Fetch currently playing song (MPD, Spotify, etc.).
- [ ] **Weather Module**: Fetch local weather.
- [ ] **More Layouts**: Add "bottom", "compact", or "horizontal" layouts.
- [ ] **Performance Optimization**: Parallelize slow hardware probes.
- [ ] **CI/CD**: Automated builds and binary releases on GitHub Actions.
- [ ] **Themes Repository**: A way to easily download community themes.
