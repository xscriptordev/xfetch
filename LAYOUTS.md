# xfetch Layouts

This document explains how to configure and customize layouts in `xfetch`.

## Built-in Layouts

### 1. Default Layout (Side-by-Side)

The default layout displays the logo (ASCII or image) on the left and the system information modules on the right.

**Configuration:**
To use this layout, simply omit the `layout` key in your config or set it to `null`.

```jsonc
{
    "layout": null,
    // ...
}
```

**Appearance:**
```
       \\\    ///    X@hostname
        \\\  ///     ---------------------------
         \\///        Ubuntu 24.04
         ///\\        6.6.87
        ///  \\\      756 (dpkg)
       ///    \\\     zsh
      ///      \\\    4.50 GiB / 16.00 GiB
```

### 2. Pac-Man Layout

A boxed layout inspired by the Pac-Man game interface. It features a top border with customizable "ghost" icons and a bottom border with custom text.

**Configuration:**
Set the `layout` key to `"pacman"`.

```jsonc
{
    "layout": "pacman",
    "header_icons": ["ᗧ", "●", "●", "●", "●"], // Icons for top border
    "footer_text": "GAME OVER"                 // Text for bottom border
}
```

**Appearance:**
```
                         ╭─ ᗧ ● ● ● ● ────────────────╮
      \\\      ///        Ubuntu 24.04
       \\\    ///         6.6.87
        \\\  ///          756 (dpkg)
         \\///            zsh
         ///\\            4.50 GiB / 16.00 GiB
        ///  \\\         
       ///    \\\        
      ///      \\\       ╰────────── GAME OVER ──────────╯
```

## Creating Custom Layouts

Currently, `xfetch` supports the predefined layouts mentioned above. However, you can heavily customize the appearance within these layouts using the configuration file.

### Customizing the Pac-Man Layout

You can change the icons in the header to anything you like (e.g., emojis, text, Nerd Font icons).

**Example: Traffic Lights**
```jsonc
{
    "layout": "pacman",
    "header_icons": ["🔴", "🟡", "🟢"],
    "footer_text": "GO!"
}
```

**Example: Minimal**
```jsonc
{
    "layout": "pacman",
    "header_icons": ["sys", "info"],
    "footer_text": "v1.0"
}
```

### Future Layouts

We plan to add more flexible layout systems in the future, allowing for:
*   Custom header/footer structures.
*   Alignment options (center, right).
*   Grid layouts.

Check the `CONFIGURATION.md` for full details on how to configure modules and colors.
