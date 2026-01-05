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
    cp configs/config.jsonc "$CONFIG_DIR/config.jsonc"
    # Copy logos if needed
    mkdir -p "$CONFIG_DIR/logos"
    cp -r logos/* "$CONFIG_DIR/logos/"
fi

# Cleanup
rm -rf "$TEMP_DIR"

# Add to PATH
add_to_path() {
    local shell_rc="$1"
    if [ -f "$shell_rc" ]; then
        if ! grep -q "export PATH=\"$INSTALL_DIR:\$PATH\"" "$shell_rc" && ! grep -q "export PATH=\"\$HOME/.local/bin:\$PATH\"" "$shell_rc"; then
            echo -e "\033[1;32m Adding xfetch to PATH in $shell_rc \033[0m"
            echo "" >> "$shell_rc"
            echo "# xfetch path" >> "$shell_rc"
            echo "export PATH=\"$INSTALL_DIR:\$PATH\"" >> "$shell_rc"
        else
            echo -e "\033[1;30m Path already configured in $shell_rc \033[0m"
        fi
    fi
}

add_xdg_config() {
    local shell_rc="$1"
    if [ -f "$shell_rc" ]; then
        if ! grep -q "export XDG_CONFIG_HOME=\"$HOME/.config\"" "$shell_rc"; then
            echo -e "\033[1;32m Adding XDG_CONFIG_HOME to $shell_rc \033[0m"
            echo "" >> "$shell_rc"
            echo "export XDG_CONFIG_HOME=\"$HOME/.config\"" >> "$shell_rc"
        else
            echo -e "\033[1;30m XDG_CONFIG_HOME already configured in $shell_rc \033[0m"
        fi
    fi
}

echo -e "\033[1;34m Configuring shell environment... \033[0m"
add_to_path "$HOME/.bashrc"
add_to_path "$HOME/.zshrc"
# Add XDG_CONFIG_HOME to shell configurations
add_xdg_config "$HOME/.bashrc"
add_xdg_config "$HOME/.zshrc"
# macOS specific
if [[ "$OSTYPE" == "darwin"* ]]; then
    add_to_path "$HOME/.bash_profile"
    add_to_path "$HOME/.zprofile"
    if [ ! -e "$HOME/Library/Application Support/xfetch" ]; then
        echo -e "\033[1;34m Creating macOS config symlink... \033[0m"
        ln -s "$HOME/.config/xfetch" "$HOME/Library/Application Support/xfetch"
    else
        echo -e "\033[1;30m macOS config path already exists \033[0m"
    fi
fi

echo -e "\033[1;32m Installation complete! \033[0m"
echo -e "You may need to restart your terminal or run 'source ~/.bashrc' (or ~/.zshrc) to use xfetch."
