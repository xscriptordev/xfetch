# xfetch Layouts

This document explains how to configure and customize layouts in `xfetch`.

## Built-in Layouts

### 1. Default Layout (Classic)

The default layout displays the logo (ASCII or image) on the left and the system information modules on the right.

**Configuration:**
To use this layout, simply omit the `layout` key in your config or set it to `null`.

```jsonc
{
    "layout": null,
    // ...
}
```

### 2. Side Block Layout

A structured layout where keys and values are displayed in two separate side-by-side boxes.

**Configuration:**
```jsonc
{
    "layout": "side-block"
}
```

**Appearance:**
```
╭──────────╮ ╭──────────────────────╮
│ User     │ │ jan.rex              │
│ Host     │ │ DIO-LAPTOP           │
╰──────────╯ ╰──────────────────────╯
```

### 3. Tree Layout

Displays modules in a hierarchical tree structure. This layout supports grouping modules.

**Configuration:**
```jsonc
{
    "layout": "tree",
    "modules": [
        {
            "type": "group",
            "title": "OS",
            "modules": ["os", "kernel", "packages"]
        },
        {
            "type": "group",
            "title": "PC",
            "modules": ["cpu", "gpu", "memory"]
        }
    ]
}
```

**Appearance:**
```
 OS
├── os Arch Linux
├── kernel 6.6.1
└── packages 1200 (pacman)
 PC
├── cpu AMD Ryzen 9
└── memory 16 GiB
```

### 4. Section Layout

Displays modules in groups with clear section headers.

**Configuration:**
```jsonc
{
    "layout": "section",
    "modules": [
        {
            "type": "group",
            "title": "Hardware",
            "modules": ["cpu", "gpu", "memory"]
        },
        {
            "type": "group",
            "title": "Software",
            "modules": ["os", "shell"]
        }
    ]
}
```

**Appearance:**
```
────── Hardware ──────
│ cpu: AMD Ryzen 9
│ memory: 16 GiB

────── Software ──────
│ os: Arch Linux
```

### 5. Pac-Man Layout

A boxed layout inspired by the Pac-Man game interface.

**Configuration:**
```jsonc
{
    "layout": "pacman",
    "header_icons": ["ᗧ", "●", "●", "●", "●"], // Icons for top border
    "footer_text": "GAME OVER"                 // Text for bottom border
}
```

### 6. Box Layout

Displays the system information enclosed in a simple box with rounded corners.

**Configuration:**
```jsonc
{
    "layout": "box"
}
```

### 7. Line Layout

Displays system information with a horizontal separator line after every 3 modules.

**Configuration:**
```jsonc
{
    "layout": "line"
}
```

### 8. Dots Layout

Similar to the Line layout, but uses dots as separators.

**Configuration:**
```jsonc
{
    "layout": "dots"
}
```

### 9. Bottom Line Layout

A minimal layout that adds a single horizontal line at the very bottom of the information list.

**Configuration:**
```jsonc
{
    "layout": "bottom_line"
}
```

## Module Grouping

For `tree` and `section` layouts, you can define groups in the `modules` list:

```jsonc
"modules": [
    // Simple module
    "uptime",
    
    // Group
    {
        "type": "group",
        "title": "Group Title",
        "modules": [
            "os",
            "kernel"
            // You can nest groups too!
        ]
    }
]
```
