use crate::config::Config;
use crate::info::Info;
use crossterm::style::{Color, Print, ResetColor, SetForegroundColor, SetBackgroundColor};
use crossterm::execute;
use std::io::stdout;
use viuer::{print_from_file, Config as ViuerConfig};
use std::path::PathBuf;

fn expand_path(path: &str) -> PathBuf {
    if path.starts_with("~") {
        if let Some(home) = dirs::home_dir() {
            return home.join(&path[2..]);
        }
    }
    PathBuf::from(path)
}

pub fn draw(info: &Info, config: &Config) {
    let mut stdout = stdout();

    // Check layout type
    let layout_type = config.layout.as_deref().unwrap_or("default");
    let is_pacman = layout_type == "pacman";
    let is_box = layout_type == "box";
    let is_line = layout_type == "line";
    let is_dots = layout_type == "dots";
    let is_bottom_line = layout_type == "bottom_line";

    // Prepare Info Items
    let info_items = prepare_info(info, config, is_pacman || is_box || is_line || is_dots || is_bottom_line);

    // ASCII/Image handling
    let mut ascii_lines: Vec<String> = Vec::new();
    let mut image_printed = false;
    let mut ascii_width = 0;

    if let Some(path_str) = &config.logo_path {
        let path = expand_path(path_str);
        // Try to print as image if extension suggests it
        if path_str.ends_with(".png") || path_str.ends_with(".jpg") || path_str.ends_with(".jpeg") || path_str.ends_with(".svg") {
            // Use viuer to print image
            // We need to print it on the left side, which is tricky with viuer as it prints to stdout directly.
            // viuer doesn't easily support side-by-side text without cursor manipulation.
            // For simplicity in this CLI tool, if image is used, we might print it above or use cursor movement.
            // But standard fetch tools usually do side-by-side.
            // To do side-by-side with viuer:
            // 1. Print image.
            // 2. Move cursor up.
            // 3. Print text with offset.
            
            // Let's try to print image first
            let conf = ViuerConfig {
                absolute_offset: false,
                transparent: true,
                ..Default::default()
            };
            
            // Print image
            if let Ok((width, height)) = print_from_file(&path, &conf) {
                image_printed = true;
                ascii_width = width as usize;
                // Move cursor up by height
                execute!(stdout, crossterm::cursor::MoveUp(height as u16)).unwrap();
            }
        } else {
             // Treat as text/ascii
             if let Ok(content) = std::fs::read_to_string(&path) {
                 for line in content.lines() {
                     ascii_lines.push(line.to_string());
                 }
             }
        }
    } else if let Some(path_str) = &config.ascii {
        let path = expand_path(path_str);
        if let Ok(content) = std::fs::read_to_string(&path) {
             for line in content.lines() {
                 ascii_lines.push(line.to_string());
             }
        }
    } else {
        // Default ASCII
        let default_art = get_default_ascii();
        for line in default_art.lines() {
            ascii_lines.push(line.to_string());
        }
    }

    if !image_printed && !ascii_lines.is_empty() {
        ascii_width = ascii_lines.iter().map(|l| l.len()).max().unwrap_or(0);
    }

    // Info preparation
    // Pacman layout specific:
    // Header: ╭ [Icons] ╮
    // Footer: ╰── X ──╯
    
    let info_height = info_items.len();
    let total_height = if is_pacman || is_box {
        info_height + 2 // +2 for borders
    } else if is_bottom_line {
        info_height + 1 // +1 for bottom line
    } else {
        info_height
    };

    let max_lines = std::cmp::max(ascii_lines.len(), total_height);
    
    // Gap between logo and info
    let gap = "  ";

    for i in 0..max_lines {
        // 1. Print Logo Part
        if image_printed {
            execute!(stdout, crossterm::cursor::MoveRight(ascii_width as u16)).unwrap();
            execute!(stdout, Print(gap)).unwrap();
        } else {
            let ascii_line = if i < ascii_lines.len() {
                &ascii_lines[i]
            } else {
                ""
            };
            
            // Print ASCII line
            let is_custom_ascii = config.ascii.is_some() || config.logo_path.is_some();
            if is_custom_ascii {
                 execute!(stdout, Print(format!("{:<width$}", ascii_line, width = ascii_width))).unwrap();
            } else {
                 execute!(
                    stdout,
                    SetForegroundColor(Color::Rgb { r: 255, g: 165, b: 0 }), // Orange default
                    Print(format!("{:<width$}", ascii_line, width = ascii_width)),
                    ResetColor
                ).unwrap();
            }
            execute!(stdout, Print(gap)).unwrap();
        }

        // 2. Print Info Part
        if is_pacman {
            if i == 0 {
                // Top Border with Icons
                // ╭─   ... ──╮
                let icons = config.header_icons.as_ref().map(|v| v.clone()).unwrap_or_default();
                execute!(stdout, SetForegroundColor(Color::Green), Print("╭─ ")).unwrap();
                for (idx, icon) in icons.iter().enumerate() {
                    // Rainbow colors for ghosts
                    let color = match idx % 5 {
                        0 => Color::Yellow, // Pacman
                        1 => Color::Red,    // Blinky
                        2 => Color::Magenta,// Pinky
                        3 => Color::Cyan,   // Inky
                        4 => Color::Rgb{r: 255, g: 165, b: 0}, // Clyde (Orange)
                        _ => Color::White,
                    };
                    execute!(stdout, SetForegroundColor(color), Print(format!("{} ", icon))).unwrap();
                }
                execute!(stdout, SetForegroundColor(Color::Green), Print("────────────────╮"), ResetColor, Print("\n")).unwrap();
            } else if i == total_height - 1 {
                // Bottom Border with Footer
                // ╰────── X ──────╯
                let footer = config.footer_text.as_deref().unwrap_or("X");
                execute!(
                    stdout, 
                    SetForegroundColor(Color::Green), 
                    Print("╰────────── "), 
                    SetForegroundColor(Color::Green), // or White?
                    Print(footer),
                    SetForegroundColor(Color::Green),
                    Print(" ──────────╯"), 
                    ResetColor, 
                    Print("\n")
                ).unwrap();
            } else {
                // Info Content
                let item_idx = i - 1;
                if item_idx < info_items.len() {
                    let (key, value) = &info_items[item_idx];
                     // Render Info Line
                     print_info_line(&mut stdout, key, value, config);
                } else {
                    execute!(stdout, Print("\n")).unwrap();
                }
            }
        } else if is_box {
            if i == 0 {
                execute!(stdout, SetForegroundColor(Color::White), Print("╭──────────────────────────────╮"), ResetColor, Print("\n")).unwrap();
            } else if i == total_height - 1 {
                execute!(stdout, SetForegroundColor(Color::White), Print("╰──────────────────────────────╯"), ResetColor, Print("\n")).unwrap();
            } else {
                let item_idx = i - 1;
                if item_idx < info_items.len() {
                    let (key, value) = &info_items[item_idx];
                    execute!(stdout, SetForegroundColor(Color::White), Print("│ "), ResetColor).unwrap();
                    print_info_line_no_newline(&mut stdout, key, value, config);
                    // Padding logic would be needed for perfect box, simplified for now
                    // Just closing the box roughly or leaving open
                    // For perfect alignment, we need to know max width of content.
                    // Assuming user content fits or we just print without right border for now or fixed width.
                    // Let's just print newline for now as right border alignment is complex without pre-calc.
                    // execute!(stdout, SetForegroundColor(Color::White), Print(" │"), ResetColor, Print("\n")).unwrap();
                    execute!(stdout, Print("\n")).unwrap();
                } else {
                    execute!(stdout, SetForegroundColor(Color::White), Print("│                              │"), ResetColor, Print("\n")).unwrap();
                }
            }
        } else if is_line {
             let item_idx = i;
             if item_idx < info_items.len() {
                let (key, value) = &info_items[item_idx];
                print_info_line(&mut stdout, key, value, config);
                if (item_idx + 1) % 3 == 0 && item_idx != info_items.len() - 1 {
                    execute!(stdout, SetForegroundColor(Color::DarkGrey), Print("──────────────────────────────"), ResetColor, Print("\n")).unwrap();
                }
            } else {
                execute!(stdout, Print("\n")).unwrap();
            }
        } else if is_dots {
             let item_idx = i;
             if item_idx < info_items.len() {
                let (key, value) = &info_items[item_idx];
                print_info_line(&mut stdout, key, value, config);
                if (item_idx + 1) % 3 == 0 && item_idx != info_items.len() - 1 {
                    execute!(stdout, SetForegroundColor(Color::DarkGrey), Print(".............................."), ResetColor, Print("\n")).unwrap();
                }
            } else {
                execute!(stdout, Print("\n")).unwrap();
            }
        } else if is_bottom_line {
             if i < info_items.len() {
                let (key, value) = &info_items[i];
                print_info_line(&mut stdout, key, value, config);
            } else if i == info_items.len() {
                 execute!(stdout, SetForegroundColor(Color::White), Print("──────────────────────────────"), ResetColor, Print("\n")).unwrap();
            } else {
                execute!(stdout, Print("\n")).unwrap();
            }
        } else {
            // Classic Layout
             if i < info_items.len() {
                let (key, value) = &info_items[i];
                print_info_line(&mut stdout, key, value, config);
            } else {
                execute!(stdout, Print("\n")).unwrap();
            }
        }
    }
}

fn print_info_line_no_newline(stdout: &mut std::io::Stdout, key: &str, value: &str, config: &Config) {
    if key == "header" {
         execute!(stdout, Print(value)).unwrap();
    } else if key == "sep" {
         execute!(stdout, Print(value)).unwrap();
    } else if key == "palette" {
         print_palette_no_newline(stdout, config);
    } else {
        // Get Icon and Color
        let icon = config.icons.get(key).map(|s| s.as_str()).unwrap_or("●");
        let color_str = config.colors.get(key).map(|s| s.as_str()).unwrap_or("White");
        let color = parse_color(color_str);

        execute!(
            stdout,
            SetForegroundColor(color),
            Print(format!("{} ", icon)),
            ResetColor,
            Print(format!("{} ", value))
        ).unwrap();
    }
}

fn print_palette_no_newline(stdout: &mut std::io::Stdout, config: &Config) {
    let colors = [
        Color::Black, Color::Red, Color::Green, Color::Yellow,
        Color::Blue, Color::Magenta, Color::Cyan, Color::White,
    ];
    let bright_colors = [
        Color::DarkGrey, Color::DarkRed, Color::DarkGreen, Color::DarkYellow,
        Color::DarkBlue, Color::DarkMagenta, Color::DarkCyan, Color::Grey,
    ];

    let style = config.palette_style.as_deref().unwrap_or("squares");
    
    // Icon (Palette emoji or custom)
    let icon = config.icons.get("palette").map(|s| s.as_str()).unwrap_or("🎨");
    let color_str = config.colors.get("palette").map(|s| s.as_str()).unwrap_or("White");
    let color = parse_color(color_str);

    execute!(
        stdout,
        SetForegroundColor(color),
        Print(format!("{} ", icon)),
        ResetColor
    ).unwrap();

    match style {
        "dots" => {
            for c in colors.iter().chain(bright_colors.iter()) {
                execute!(stdout, SetForegroundColor(*c), Print("● ")).unwrap();
            }
        },
        "squares" => {
             for c in colors.iter().chain(bright_colors.iter()) {
                execute!(stdout, SetBackgroundColor(*c), Print("  "), ResetColor, Print(" ")).unwrap();
            }
        },
        "lines" => {
            for c in colors.iter().chain(bright_colors.iter()) {
                execute!(stdout, SetBackgroundColor(*c), Print("   ")).unwrap();
            }
            execute!(stdout, ResetColor).unwrap();
        },
        "triangles" => {
             for c in colors.iter().chain(bright_colors.iter()) {
                execute!(stdout, SetForegroundColor(*c), Print("▲ ")).unwrap();
            }
        }
        _ => {
            // Default to squares
             for c in colors.iter().chain(bright_colors.iter()) {
                execute!(stdout, SetBackgroundColor(*c), Print("  "), ResetColor, Print(" ")).unwrap();
            }
        }
    }
}

fn print_info_line(stdout: &mut std::io::Stdout, key: &str, value: &str, config: &Config) {
    if key == "header" {
         execute!(stdout, Print(value), Print("\n")).unwrap();
    } else if key == "sep" {
         execute!(stdout, Print(value), Print("\n")).unwrap();
    } else if key == "palette" {
         print_palette(stdout, config);
    } else {
        // Get Icon and Color
        let icon = config.icons.get(key).map(|s| s.as_str()).unwrap_or("●");
        let color_str = config.colors.get(key).map(|s| s.as_str()).unwrap_or("White");
        let color = parse_color(color_str);

        execute!(
            stdout,
            SetForegroundColor(color),
            Print(format!("{} ", icon)),
            ResetColor,
            Print(format!("{} ", value)),
            Print("\n")
        ).unwrap();
    }
}

fn print_palette(stdout: &mut std::io::Stdout, config: &Config) {
    let colors = [
        Color::Black, Color::Red, Color::Green, Color::Yellow,
        Color::Blue, Color::Magenta, Color::Cyan, Color::White,
    ];
    let bright_colors = [
        Color::DarkGrey, Color::DarkRed, Color::DarkGreen, Color::DarkYellow,
        Color::DarkBlue, Color::DarkMagenta, Color::DarkCyan, Color::Grey,
    ];

    let style = config.palette_style.as_deref().unwrap_or("squares");
    
    // Icon (Palette emoji or custom)
    let icon = config.icons.get("palette").map(|s| s.as_str()).unwrap_or("🎨");
    let color_str = config.colors.get("palette").map(|s| s.as_str()).unwrap_or("White");
    let color = parse_color(color_str);

    execute!(
        stdout,
        SetForegroundColor(color),
        Print(format!("{} ", icon)),
        ResetColor
    ).unwrap();

    match style {
        "dots" => {
            for c in colors.iter().chain(bright_colors.iter()) {
                execute!(stdout, SetForegroundColor(*c), Print("● ")).unwrap();
            }
        },
        "squares" => {
             for c in colors.iter().chain(bright_colors.iter()) {
                execute!(stdout, SetBackgroundColor(*c), Print("  "), ResetColor, Print(" ")).unwrap();
            }
        },
        "lines" => {
            for c in colors.iter().chain(bright_colors.iter()) {
                execute!(stdout, SetBackgroundColor(*c), Print("   ")).unwrap();
            }
            execute!(stdout, ResetColor).unwrap();
        },
        "triangles" => {
             for c in colors.iter().chain(bright_colors.iter()) {
                execute!(stdout, SetForegroundColor(*c), Print("▲ ")).unwrap();
            }
        }
        _ => {
            // Default to squares
             for c in colors.iter().chain(bright_colors.iter()) {
                execute!(stdout, SetBackgroundColor(*c), Print("  "), ResetColor, Print(" ")).unwrap();
            }
        }
    }
    execute!(stdout, Print("\n")).unwrap();
}

fn get_default_ascii() -> String {
    r#"
      \\\      ///
       \\\    ///
        \\\  ///
         \\///
         ///\\
        ///  \\\
       ///    \\\
      ///      \\\
"#.trim().to_string()
}

fn prepare_info(info: &Info, config: &Config, is_pacman: bool) -> Vec<(String, String)> {
    let mut items = Vec::new();

    // User header
    if !is_pacman {
         items.push(("header".to_string(), format!("{}@{}", "X", info.host_name)));
         items.push(("sep".to_string(), "---------------------------".to_string()));
    } else {
         // In Pacman layout, Header is inside the box, usually first line
         // Removed hardcoded "XOs Linux x86_64" as requested
         // items.push(("header".to_string(), format!("  XOs Linux x86_64"))); 
    }

    // Config modules
    for module in &config.modules {
        let val = match module.as_str() {
            "os" => Some(info.os.clone()),
            "kernel" => Some(info.kernel.clone()),
            "hostname" => Some(info.host_name.clone()),
            "wm" => Some(info.desktop.clone()),
            "packages" => Some(info.packages.clone()),
            "shell" => Some(info.shell.clone()),
            "cpu" => Some(info.cpu.clone()),
            "gpu" => {
                if info.gpu.is_empty() { Some("Unknown".to_string()) }
                else { Some(info.gpu.join(" / ")) }
            },
            "memory" => Some(info.memory.clone()),
            "swap" => Some(info.swap.clone()),
            "disk" => {
                 if info.disks.is_empty() { Some("Unknown".to_string()) }
                 else { Some(info.disks[0].clone()) } 
            },
            "battery" => Some(info.battery.clone()),
            "uptime" => Some(info.uptime.clone()),
            "terminal" => Some(info.terminal.clone()),
            "palette" => Some("palette".to_string()), // Placeholder, handled in print_info_line
            _ => None,
        };

        if let Some(v) = val {
            items.push((module.clone(), v));
        }
    }

    items
}

fn parse_color(color: &str) -> Color {
    match color.to_lowercase().as_str() {
        "black" => Color::Black,
        "red" => Color::Red,
        "green" => Color::Green,
        "yellow" => Color::Yellow,
        "blue" => Color::Blue,
        "magenta" => Color::Magenta,
        "cyan" => Color::Cyan,
        "white" => Color::White,
        "grey" | "gray" => Color::Grey,
        "darkgrey" | "darkgray" => Color::DarkGrey,
        "darkred" => Color::DarkRed,
        "darkgreen" => Color::DarkGreen,
        "darkyellow" => Color::DarkYellow,
        "darkblue" => Color::DarkBlue,
        "darkmagenta" => Color::DarkMagenta,
        "darkcyan" => Color::DarkCyan,
        _ => Color::White,
    }
}
