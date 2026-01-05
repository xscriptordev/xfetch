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

### 2. Pac-Man Layout

A boxed layout inspired by the Pac-Man game interface.

**Configuration:**
```jsonc
{
    "layout": "pacman",
    "header_icons": ["ᗧ", "●", "●", "●", "●"], // Icons for top border
    "footer_text": "GAME OVER"                 // Text for bottom border
}
```

### 3. Box Layout

Displays the system information enclosed in a simple box with rounded corners.

**Configuration:**
```jsonc
{
    "layout": "box"
}
```

**Appearance:**
```
                         ╭──────────────────────────────╮
      \\\      ///       │  Ubuntu 24.04               
       \\\    ///        │  6.6.87                     
        \\\  ///         │  756 (dpkg)                 
         \\///           │  zsh                        
         ///\\           │  4.50 GiB / 16.00 GiB       
        ///  \\\         │                              
       ///    \\\        │                              
      ///      \\\       ╰──────────────────────────────╯
```

### 4. Line Layout

Displays system information with a horizontal separator line after every 3 modules. Useful for grouping related information.

**Configuration:**
```jsonc
{
    "layout": "line"
}
```

**Appearance:**
```
      \\\      ///        Ubuntu 24.04
       \\\    ///         6.6.87
        \\\  ///          756 (dpkg)
         \\///           ──────────────────────────────
         ///\\            zsh
        ///  \\\          4.50 GiB / 16.00 GiB
       ///    \\\        
      ///      \\\       
```

### 5. Dots Layout

Similar to the Line layout, but uses dots as separators between groups of 3 modules.

**Configuration:**
```jsonc
{
    "layout": "dots"
}
```

**Appearance:**
```
      \\\      ///        Ubuntu 24.04
       \\\    ///         6.6.87
        \\\  ///          756 (dpkg)
         \\///           ..............................
         ///\\            zsh
        ///  \\\          4.50 GiB / 16.00 GiB
       ///    \\\        
      ///      \\\       
```

### 6. Bottom Line Layout

A minimal layout that adds a single horizontal line at the very bottom of the information list.

**Configuration:**
```jsonc
{
    "layout": "bottom_line"
}
```

**Appearance:**
```
      \\\      ///        Ubuntu 24.04
       \\\    ///         6.6.87
        \\\  ///          756 (dpkg)
         \\///            zsh
         ///\\            4.50 GiB / 16.00 GiB
        ///  \\\         ──────────────────────────────
       ///    \\\        
      ///      \\\       
```

## Creating Custom Layouts

Currently, `xfetch` supports the predefined layouts mentioned above. However, you can customize components like header icons and footer text within supported layouts (like `pacman`).

Check the `CONFIGURATION.md` for full details on how to configure modules and colors.
