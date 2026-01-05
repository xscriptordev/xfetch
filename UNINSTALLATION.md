# Uninstalling xfetch

We are sorry to see you go! If you encountered any issues, please feel free to open an issue on our GitHub repository.

## Quick Uninstall

You can uninstall xfetch by running the following command in your terminal:

```bash
curl -fsSL https://raw.githubusercontent.com/xscriptordev/xfetch/main/uninstall.sh | bash
```

## Manual Uninstall

If you prefer to uninstall manually, you can delete the following files and directories:

1.  **Binary**: Remove the executable.
    ```bash
    rm ~/.local/bin/xfetch
    ```

2.  **Configuration**: Remove the configuration directory.
    ```bash
    rm -rf ~/.config/xfetch
    ```

3.  **Shell Config**: Open your shell configuration file (`~/.bashrc`, `~/.zshrc`, or `~/.bash_profile`) and remove the lines added by the installer:

    ```bash
    # xfetch path
    export PATH="$HOME/.local/bin:$PATH"
    ```
