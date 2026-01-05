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

## Logos and ASCII Art

You can display custom logos using text files or images.

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
