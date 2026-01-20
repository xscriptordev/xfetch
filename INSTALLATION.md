# Installation Guide

This guide covers the installation process for **xfetch** on various operating systems.

## Prerequisites

**xfetch** is built with Rust. You must have the Rust toolchain installed on your system.

To install Rust, run the following command (works on Linux and macOS):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

For Windows, download the installer from [rust-lang.org](https://www.rust-lang.org/tools/install).

---

## Linux

### Debian/Ubuntu based (Debian, Ubuntu, Linux Mint, Pop!_OS)

1.  **Install dependencies**:
    You may need development tools and libraries for image processing.
    ```bash
    sudo apt update
    sudo apt install build-essential pkg-config libssl-dev
    ```

2.  **Clone and Install**:
    ```bash
    git clone https://github.com/xscriptordev/xfetch.git
    cd xfetch
    cargo install --path .
    ```

3.  **Add to PATH**:
    Ensure `~/.cargo/bin` is in your PATH.

### Arch Linux based (Arch, Manjaro, EndeavourOS)

1.  **Install dependencies**:
    ```bash
    sudo pacman -S base-devel
    ```

2.  **Clone and Install**:
    ```bash
    git clone https://github.com/xscriptordev/xfetch.git
    cd xfetch
    cargo install --path .
    ```

### Using PKGBUILD (Recommended)

This method installs xfetch as a proper Arch package, making it easy to update and remove.

1.  **Clone the repository**:
    ```bash
    git clone https://github.com/xscriptordev/xfetch.git
    cd xfetch
    ```

2.  **Build and install the package**:
    ```bash
    makepkg -si
    ```
    This will:
    - Download dependencies automatically
    - Build the package from source
    - Install it system-wide to `/usr/bin/xfetch`

3.  **Verify installation**:
    ```bash
    xfetch
    ```

> **Note:** Config examples and logos are installed to `/usr/share/xfetch/`.

To uninstall:
```bash
sudo pacman -R xfetch-git
```

---

## macOS

1.  **Install dependencies** (optional, usually handled by cargo/homebrew):
    Ensure you have Xcode Command Line Tools installed:
    ```bash
    xcode-select --install
    ```

2.  **Clone and Install**:
    ```bash
    git clone https://github.com/xscriptordev/xfetch.git
    cd xfetch
    cargo install --path .
    ```

---

## Windows

1.  **Prerequisites**:
    Ensure you have the Visual Studio C++ Build Tools installed (usually part of the Rust installation setup).

2.  **Clone and Install**:
    Open PowerShell or Command Prompt:
    ```powershell
    git clone https://github.com/xscriptordev/xfetch.git
    cd xfetch
    cargo install --path .
    ```

3.  **Running**:
    You can now run `xfetch` from any terminal window.

---

## Verify Installation

After installation, verify it works by running:

```bash
xfetch
```

Or test a specific config:

```bash
xfetch --config configs/config_11_pacman.jsonc
```
