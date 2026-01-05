#!/bin/bash
# xfetch uninstaller script

INSTALL_DIR="$HOME/.local/bin"
BINARY="$INSTALL_DIR/xfetch"
CONFIG_DIR="$HOME/.config/xfetch"

echo -e "\033[1;36m Uninstalling xfetch... \033[0m"

# Remove Binary
if [ -f "$BINARY" ]; then
    rm "$BINARY"
    echo -e "\033[1;32m Removed binary: $BINARY \033[0m"
else
    echo -e "\033[1;33m Binary not found at $BINARY \033[0m"
fi

# Remove Config
if [ -d "$CONFIG_DIR" ]; then
    rm -rf "$CONFIG_DIR"
    echo -e "\033[1;32m Removed config directory: $CONFIG_DIR \033[0m"
else
    echo -e "\033[1;33m Config directory not found at $CONFIG_DIR \033[0m"
fi

echo -e "\033[1;36m Uninstallation finished. \033[0m"
echo "Note: The PATH export line in your .bashrc/.zshrc was NOT removed automatically."
echo "You can manually remove the following lines from your shell config file:"
echo ""
echo "# xfetch path"
echo "export PATH=\"$INSTALL_DIR:\$PATH\""
