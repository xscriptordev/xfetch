use crate::config::Config;
use crate::info::Info;
use crossterm::style::{Color, Print, ResetColor, SetForegroundColor};
use crossterm::execute;
use std::io::stdout;
use viuer::{print_from_file, Config as ViuerConfig};

pub fn draw(info: &Info, config: &Config) {
    let mut stdout = stdout();

    // Check if layout is "pacman"
    let is_pacman = config.layout.as_deref() == Some("pacman");

    // Prepare Info Items
    let info_items = prepare_info(info, config, is_pacman);

    // ASCII/Image handling
    let mut ascii_lines: Vec<String> = Vec::new();
    let mut image_printed = false;
    let mut ascii_width = 0;

    if let Some(path) = &config.logo_path {
        // Try to print as image if extension suggests it
        if path.ends_with(".png") || path.ends_with(".jpg") || path.ends_with(".jpeg") {
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
            if let Ok((width, height)) = print_from_file(path, &conf) {
                image_printed = true;
                ascii_width = width as usize;
                // Move cursor up by height
                execute!(stdout, crossterm::cursor::MoveUp(height as u16)).unwrap();
            }
        } else {
             // Treat as text/ascii
             if let Ok(content) = std::fs::read_to_string(path) {
                 for line in content.lines() {
                     ascii_lines.push(line.to_string());
                 }
             }
        }
    } else if let Some(path) = &config.ascii {
        if let Ok(content) = std::fs::read_to_string(path) {
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
    let total_height = if is_pacman {
        info_height + 2 // +2 for borders
    } else {
        info_height
    };

    let max_lines = std::cmp::max(ascii_lines.len(), total_height);
    
    // Gap between logo and info
    let gap = "  ";

    for i in 0..max_lines {
        // 1. Print Logo Part
        if image_printed {
            // If image was printed, we just need to move cursor to the right column?
            // Actually, if we moved cursor up, we are at the top-left of the image.
            // We need to print spaces to skip the image width.
            // But wait, print_from_file prints blocks.
            // We need to move cursor right by ascii_width for each line.
            // execute!(stdout, crossterm::cursor::MoveRight(ascii_width as u16)).unwrap();
            // This is complex to sync. 
            // Simplified approach for image: Print image, then print info below?
            // Or use cursor save/restore.
            // Let's stick to simple space padding for now, assuming cursor is at start of line.
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
                // Calculate width to match top roughly or fixed
                // Top width depends on icons. Let's make it fixed or based on content?
                // For now fixed length line
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

fn print_info_line(stdout: &mut std::io::Stdout, key: &str, value: &str, config: &Config) {
    if key == "header" {
         execute!(stdout, Print(value), Print("\n")).unwrap();
    } else if key == "sep" {
         execute!(stdout, Print(value), Print("\n")).unwrap();
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

fn get_default_ascii() -> String {
    r#"
*****************************************
*****************************************
************************#%***************
**************************%0%************
*********#%#########%********#*****%*****
*******%****#@@@@@@****#*****%%%%%%%%****
*****%@@#****@@@@@##**%%%%%%0%%%0%*******
*****@@@@%%***%@@@%%***###***************
******@@%%%%****%@@@%#*******************
*******#*********%@@@%#******************
*******************%@@@%*****************
*******************%*****%@@@@@**********
***********************##000%%***********
***********************%@@@@*************
************************#****#%@@@@@*****
************************%@@@@@***********
*************************%@@@@@%*********
*******%@@@@@@@@%*******%%%@@@@@%%#******
******%@@%@@@@@#*****%****%@@@%#*********
****#***************#@@@@@#**************
****#***************#@*******************
****#*******@****************************
******####**********#********************
********************0s->powered by Arch**
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
         items.push(("header".to_string(), format!("  XOs Linux x86_64"))); // Hardcoded style from screenshot
         // items.push(("sep".to_string(), "".to_string())); // No separator in screenshot?
    }

    // Config modules
    for module in &config.modules {
        let val = match module.as_str() {
            "os" => Some(info.os.clone()),
            "kernel" => Some(info.kernel.clone()),
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
