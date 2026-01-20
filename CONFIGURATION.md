# Configuration Guide

**xfetch** is highly customizable using JSONC (JSON with Comments) files. This guide explains how to customize every aspect of the tool.

## Config File Location

By default, xfetch looks for a configuration file at:
*   **Linux**: `~/.config/xfetch/config.jsonc`
*   **Windows**: `%APPDATA%\xfetch\config.jsonc`
*   **macOS**: `~/Library/Application Support/xfetch/config.jsonc`

You can also pass a custom config file using the `--config` flag:
```bash
xfetch --config path/to/my_config.jsonc
```

## Basic Structure

A minimal configuration looks like this:

```jsonc
{
    "modules": ["os", "kernel", "memory"],
    "show_colors": true
}
```

## Customizing Modules

The `modules` array determines which information is displayed and in what order.

**Available Modules:**
*   `os`: Operating System name and architecture
*   `kernel`: Kernel version
*   `hostname`: Hostname of the machine
*   `uptime`: System uptime
*   `packages`: Package count (pacman, dpkg, brew, scoop, etc.)
*   `shell`: Current shell (bash, zsh, powershell, etc.)
*   `terminal`: Current terminal emulator
*   `wm`: Window Manager / Desktop Environment
*   `cpu`: CPU model and frequency
*   `gpu`: GPU model
*   `memory`: RAM usage
*   `swap`: Swap memory usage
*   `disk`: Disk usage (first disk)
*   `battery`: Battery percentage and status
*   `palette`: Color palette

## Logos and ASCII Art

You can display custom logos using text files or images.

### Color System for ASCII Logos

xfetch supports two methods for coloring ASCII logos:

#### 1. ANSI Escape Codes in Custom ASCII Files

When using a custom ASCII logo file (via `logo_path` or `ascii`), you can embed **ANSI escape codes** directly in the text file to add colors. The escape codes are interpreted by the terminal to render colored text.

**Format:** `\x1b[<code>m` or `\033[<code>m`

**Available Foreground Color Codes:**

| Color    | Code | Example                    |
|----------|------|----------------------------|
| Black    | 30   | `\x1b[30mText\x1b[0m`       |
| Red      | 31   | `\x1b[31mText\x1b[0m`       |
| Green    | 32   | `\x1b[32mText\x1b[0m`       |
| Yellow   | 33   | `\x1b[33mText\x1b[0m`       |
| Blue     | 34   | `\x1b[34mText\x1b[0m`       |
| Magenta  | 35   | `\x1b[35mText\x1b[0m`       |
| Cyan     | 36   | `\x1b[36mText\x1b[0m`       |
| White    | 37   | `\x1b[37mText\x1b[0m`       |
| Gray     | 90   | `\x1b[90mText\x1b[0m`       |

**256-Color Mode:** `\x1b[38;5;<n>m` where `<n>` is 0-255

**RGB True Color:** `\x1b[38;2;<r>;<g>;<b>m`

**Reset Code:** `\x1b[0m` (resets all formatting)

**Example ASCII Logo with Colors (`x_logo.txt`):**
```
\x1b[36m      \\\\\\      ///
\x1b[36m       \\\\\\    ///
\x1b[35m        \\\\\\  ///
\x1b[35m         \\\\///
\x1b[33m         ///\\\\
\x1b[33m        ///  \\\\\\
\x1b[32m       ///    \\\\\\
\x1b[32m      ///      \\\\\\
```

This creates a gradient effect from cyan to green.

#### 2. Default ASCII Logo Color

When **no custom logo is specified**, xfetch uses a built-in default ASCII logo. This logo is rendered with an **orange color** (`RGB: 255, 165, 0`) applied programmatically.

The color is set in `src/ui.rs` using CrossTerm:
```rust
SetForegroundColor(Color::Rgb { r: 255, g: 165, b: 0 })
```

> **Note:** Custom ASCII logos bypass this automatic coloring and use their embedded ANSI codes instead.

### Text/ASCII Logos
Create a text file (e.g., `logo.txt`). You can use ANSI escape codes for colors in this file.

```jsonc
{
    // You can use tilde (~) for home directory
    "logo_path": "~/.config/xfetch/logos/arch.txt",
    // ...
}
```

### Images
xfetch supports displaying images (png, jpg, svg) if your terminal supports it (using protocols like iTerm2, Kitty, or Sixel, handled by `viuer`).

```jsonc
{
    "logo_path": "/path/to/logo.png",
    // ...
}
```

## Layouts

### Default Layout
The standard "side-by-side" fetch layout.

```jsonc
{
    "layout": null // or omit this field
}
```

### Pac-Man Layout
A boxed layout with a custom header and footer, inspired by Pac-Man.

```jsonc
{
    "layout": "pacman",
    // Icons displayed in the top border
    "header_icons": ["ᗧ", "●", "●", "●"], 
    // Text displayed in the bottom border
    "footer_text": "GAME OVER"
}
```

## Icons and Emojis

You can customize the icon displayed next to each module. You can use standard Emojis or Nerd Fonts.

```jsonc
{
    "icons": {
        "os": "",      // Arch Linux icon (Nerd Font)
        "cpu": "🧠",    // Brain emoji
        "memory": "RAM:" // Plain text
    }
}
```

## Colors

You can set the color for the icon/label of each module.

**Available Colors:**
*   `Black`
*   `Red`
*   `Green`
*   `Yellow`
*   `Blue`
*   `Magenta`
*   `Cyan`
*   `White`
*   `Grey` (or `Gray`)
*   `DarkGrey` (or `DarkGray`)
*   `DarkRed`
*   `DarkGreen`
*   `DarkYellow`
*   `DarkBlue`
*   `DarkMagenta`
*   `DarkCyan`

```jsonc
{
    "colors": {
        "os": "Cyan",
        "cpu": "Red",
        "memory": "Green"
    }
}
```

## Full Example

```jsonc
{
    "logo_path": "~/.config/xfetch/logos/ghost.txt",
    "layout": "pacman",
    "header_icons": ["ᗧ", "ᗣ", "ᗣ"],
    "footer_text": "xfetch",
    "modules": [
        "os",
        "kernel",
        "cpu",
        "memory"
    ],
    "show_colors": true,
    "icons": {
        "os": "",
        "cpu": "",
        "memory": ""
    },
    "colors": {
        "os": "Blue",
        "cpu": "Red",
        "memory": "Yellow"
    }
}
```
