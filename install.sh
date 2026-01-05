#!/bin/bash
# xfetch installer script

set -e

REPO_URL="https://github.com/xscriptordev/xfetch"
INSTALL_DIR="$HOME/.local/bin"
CONFIG_DIR="$HOME/.config/xfetch"

echo -e "\033[1;36m Installing xfetch... \033[0m"

# Check for Rust
if ! command -v cargo &> /dev/null; then
    echo -e "\033[1;33m Rust (cargo) is not installed. Installing Rust... \033[0m"
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
fi

# Clone and Build
TEMP_DIR=$(mktemp -d)
echo -e "\033[1;34m Cloning repository... \033[0m"
git clone --depth 1 $REPO_URL "$TEMP_DIR/xfetch"

echo -e "\033[1;34m Building xfetch... \033[0m"
cd "$TEMP_DIR/xfetch"
cargo build --release

# Install Binary
echo -e "\033[1;34m Installing binary to $INSTALL_DIR... \033[0m"
mkdir -p "$INSTALL_DIR"
cp target/release/xfetch "$INSTALL_DIR/"

# Setup Config
echo -e "\033[1;34m Setting up default config... \033[0m"
mkdir -p "$CONFIG_DIR"
if [ ! -f "$CONFIG_DIR/config.jsonc" ]; then
    cp configs/config_11_pacman.jsonc "$CONFIG_DIR/config.jsonc"
    # Copy logos if needed, though they are usually paths.
    # Ideally, binary should embed default logos or we copy assets.
    mkdir -p "$CONFIG_DIR/logos"
    cp -r logos/* "$CONFIG_DIR/logos/"
fi

# Cleanup
rm -rf "$TEMP_DIR"

# Path check
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo -e "\033[1;33m Warning: $INSTALL_DIR is not in your PATH. \033[0m"
    echo "Add the following to your shell config (.bashrc, .zshrc, etc.):"
    echo "export PATH=\"\$HOME/.local/bin:\$PATH\""
fi

echo -e "\033[1;32m Installation complete! Run 'xfetch' to start. \033[0m"
