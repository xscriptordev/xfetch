# xfetch

A cross-platform system information fetching tool inspired by fastfetch and neofetch, written in Rust.

![xfetch](https://raw.githubusercontent.com/xscriptordev/xfetch/main/assets/preview.png)

## Quick Install

### Linux / macOS
```bash
curl -fsSL https://raw.githubusercontent.com/xscriptordev/xfetch/main/install.sh | bash
```

### Windows (PowerShell)
```powershell
irm https://raw.githubusercontent.com/xscriptordev/xfetch/main/install.ps1 | iex
```

## Features

- **Cross-platform**: Works on Linux, Windows, and macOS.
- **Customizable**: Configure modules via `config.jsonc`.
- **Fast**: Written in Rust for performance.

## Installation

### From Source

1. Ensure you have Rust installed.
2. Clone the repository.
3. Build and run:

```bash
cargo run --release
```

Or install locally:

```bash
cargo install --path .
```

## Configuration

xfetch looks for a configuration file at:

- **Linux**: `~/.config/xfetch/config.jsonc`
- **Windows**: `%APPDATA%\xfetch\config.jsonc`
- **macOS**: `~/Library/Application Support/xfetch/config.jsonc`

### Example Config (`config.jsonc`)

```jsonc
// Configuration for xfetch
{
  // Path to custom ASCII art file (optional)
  "ascii": null, 
  // Modules to display
  "modules": [
    "os",
    "kernel",
    "wm",
    "packages",
    "shell",
    "cpu",
    "gpu",
    "memory",
    "disk",
    "battery",
    "uptime",
    "terminal"
  ],
  // Enable colors
  "show_colors": true
}
```

## Usage

Simply run `xfetch` in your terminal.

```bash
xfetch
```

You can also specify a config file via CLI args (not yet fully implemented in CLI but supported in code structure).

## License
[MIT](LICENSE)
