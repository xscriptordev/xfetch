use crate::config::{Config, ModuleConfig};
use crate::info::Info;
use crossterm::style::{Color, Print, ResetColor, SetForegroundColor};
use crossterm::execute;
use std::io::stdout;
use viuer::{print_from_file, Config as ViuerConfig};
use std::path::PathBuf;
use console::strip_ansi_codes;

fn expand_path(path: &str) -> PathBuf {
    if path.starts_with("~") {
        if let Some(home) = dirs::home_dir() {
            return home.join(&path[2..]);
        }
    }
    PathBuf::from(path)
}

#[derive(Debug)]
enum RenderNode {
    Line { key: String, value: String, icon: String },
    Group { title: String, children: Vec<RenderNode> },
}

pub fn draw(info: &Info, config: &Config) {
    let mut stdout = stdout();

    // Prepare Render Tree
    let nodes = prepare_render_tree(info, &config.modules, config);

    // ASCII/Image handling
    let mut ascii_lines: Vec<String> = Vec::new();
    let mut image_printed = false;
    let mut ascii_width = 0;

    if let Some(path_str) = &config.logo_path {
        let path = expand_path(path_str);
        if path_str.ends_with(".png") || path_str.ends_with(".jpg") || path_str.ends_with(".jpeg") || path_str.ends_with(".svg") {
            let conf = ViuerConfig {
                absolute_offset: false,
                transparent: true,
                ..Default::default()
            };
            if let Ok((width, height)) = print_from_file(&path, &conf) {
                image_printed = true;
                ascii_width = width as usize;
                execute!(stdout, crossterm::cursor::MoveUp(height as u16)).unwrap();
            }
        } else {
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
        let default_art = get_default_ascii();
        for line in default_art.lines() {
            ascii_lines.push(line.to_string());
        }
    }

    if !image_printed && !ascii_lines.is_empty() {
        // Trim trailing spaces from ascii lines to avoid excessive width
        ascii_lines = ascii_lines.into_iter().map(|l| l.trim_end().to_string()).collect();
        // Use console::measure_text_width to get accurate display width (handling wide chars correctly)
        ascii_width = ascii_lines.iter().map(|l| console::measure_text_width(l)).max().unwrap_or(0);
    }

    // Render content to lines based on layout
    let layout_type = config.layout.as_deref().unwrap_or("default");
    let content_lines = match layout_type {
        "side-block" => render_side_block(&nodes, config),
        "tree" => render_tree(&nodes, config), // Image 2 style
        "section" => render_section(&nodes, config), // Image 3/4 style
        "pacman" | "box" | "line" | "dots" | "bottom_line" => render_classic_variants(&nodes, config, layout_type),
        _ => render_classic(&nodes, config),
    };

    let max_lines = std::cmp::max(ascii_lines.len(), content_lines.len());
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
            let is_custom_ascii = config.ascii.is_some() || config.logo_path.is_some();
            if is_custom_ascii {
                 // Calculate padding needed: width - visible_width(ascii_line)
                 let visible_len = console::measure_text_width(ascii_line);
                 let padding = if ascii_width > visible_len { ascii_width - visible_len } else { 0 };
                 execute!(stdout, Print(format!("{}{}", ascii_line, " ".repeat(padding)))).unwrap();
            } else {
                 // Calculate padding needed
                 let visible_len = console::measure_text_width(ascii_line);
                 let padding = if ascii_width > visible_len { ascii_width - visible_len } else { 0 };
                 
                 execute!(
                    stdout,
                    SetForegroundColor(Color::Rgb { r: 255, g: 165, b: 0 }),
                    Print(format!("{}{}", ascii_line, " ".repeat(padding))),
                    ResetColor
                ).unwrap();
            }
            execute!(stdout, Print(gap)).unwrap();
        }

        // 2. Print Info Part
        if i < content_lines.len() {
            execute!(stdout, Print(&content_lines[i])).unwrap();
        }
        execute!(stdout, Print("\n")).unwrap();
    }
}

fn prepare_render_tree(info: &Info, modules: &[ModuleConfig], config: &Config) -> Vec<RenderNode> {
    let mut nodes = Vec::new();
    for module in modules {
        match module {
            ModuleConfig::Simple(key) => {
                if key == "palette" {
                    let val = format_palette(config);
                    // Icon for palette is optional, can be "Colors" or empty string if user wants no icon
                    let icon = config.icons.get(key).cloned().unwrap_or("🎨".to_string());
                    nodes.push(RenderNode::Line { key: key.clone(), value: val, icon });
                } else {
                    let val = get_module_value(info, key);
                    if let Some(v) = val {
                        let icon = config.icons.get(key).cloned().unwrap_or("●".to_string());
                        nodes.push(RenderNode::Line { key: key.clone(), value: v, icon });
                    }
                }
            },
            ModuleConfig::Group { title, modules } => {
                let children = prepare_render_tree(info, modules, config);
                if !children.is_empty() {
                    nodes.push(RenderNode::Group { title: title.clone(), children });
                }
            }
        }
    }
    nodes
}

fn get_module_value(info: &Info, key: &str) -> Option<String> {
    match key {
        "os" => Some(info.os.clone()),
        "kernel" => Some(info.kernel.clone()),
        "hostname" | "host" => Some(info.host_name.clone()),
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
             else { Some(info.disks[0].clone()) } // Simplified
        },
        "battery" => Some(info.battery.clone()),
        "uptime" => Some(info.uptime.clone()),
        "terminal" => Some(info.terminal.clone()),
        "user" => Some(info.user.clone()),
        "datetime" => Some(info.datetime.clone()),
        "local_ip" => Some(info.local_ip.clone()),
        "palette" => None, // Handled in prepare_render_tree
        "header" => Some(format!("{}@{}", info.user, info.host_name)), // Custom module for header
        "sep" => Some("---".to_string()),
        _ => None,
    }
}

// --- Renderers ---

fn render_classic(nodes: &[RenderNode], config: &Config) -> Vec<String> {
    let mut lines = Vec::new();
    // Flatten
    for node in nodes {
        match node {
            RenderNode::Line { key, value, icon } => {
                lines.push(format_line(key, value, icon, config));
            },
            RenderNode::Group { title, children } => {
                lines.push(format!("-- {} --", title));
                for child in children {
                     if let RenderNode::Line { key, value, icon } = child {
                         lines.push(format_line(key, value, icon, config));
                     }
                }
            },
        }
    }
    lines
}

fn render_classic_variants(nodes: &[RenderNode], config: &Config, variant: &str) -> Vec<String> {
    let mut lines = Vec::new();
    let flat_items = flatten_nodes(nodes);
    
    match variant {
        "box" => {
             let max_len = flat_items.iter().map(|(k, v, i)| {
                let content = format_line_content(k, v, i, config);
                strip_ansi_codes(&content).chars().count()
            }).max().unwrap_or(0);
            
            let border_len = max_len + 2; // +2 for padding space
            lines.push(format!("╭{}╮", "─".repeat(border_len)));
            
            for (key, val, icon) in flat_items {
                let content = format_line_content(&key, &val, &icon, config);
                let visual_len = strip_ansi_codes(&content).chars().count();
                let padding = max_len - visual_len;
                lines.push(format!("│ {} {}│", content, " ".repeat(padding)));
            }
            lines.push(format!("╰{}╯", "─".repeat(border_len)));
        },
        "pacman" => {
             // Header
            let icons = config.header_icons.as_ref().map(|v| v.clone()).unwrap_or_default();
            let mut header = String::from("\x1b[32m╭─ \x1b[0m");
            for (idx, icon) in icons.iter().enumerate() {
                let color = match idx % 5 {
                     0 => "33", // Yellow
                     1 => "31", // Red
                     2 => "35", // Magenta
                     3 => "36", // Cyan
                     4 => "33", // Orange-ish
                     _ => "37",
                };
                header.push_str(&format!("\x1b[{}m{} \x1b[0m", color, icon));
            }
            header.push_str("\x1b[32m────────────────╮\x1b[0m");
            lines.push(header);
            
            // Content
            for (key, val, icon) in flat_items {
                 lines.push(format_line(&key, &val, &icon, config));
            }
            
            // Footer
            let footer_text = config.footer_text.as_deref().unwrap_or("X");
             lines.push(format!(
                "\x1b[32m╰────────── \x1b[37m{}\x1b[32m ──────────╯\x1b[0m", 
                footer_text
            ));
        },
        "line" | "dots" => {
            for (idx, (key, val, icon)) in flat_items.iter().enumerate() {
                lines.push(format_line(key, val, icon, config));
                if (idx + 1) % 3 == 0 && idx != flat_items.len() - 1 {
                     let sep = if variant == "line" { "──────────────────────────────" } else { ".............................." };
                     lines.push(format!("\x1b[90m{}\x1b[0m", sep));
                }
            }
        },
        "bottom_line" => {
             for (key, val, icon) in flat_items {
                lines.push(format_line(&key, &val, &icon, config));
            }
            lines.push("\x1b[37m──────────────────────────────\x1b[0m".to_string());
        },
        _ => return render_classic(nodes, config),
    }
    lines
}

// Image 1: Side Block
fn render_side_block(nodes: &[RenderNode], config: &Config) -> Vec<String> {
    let mut lines = Vec::new();
    let flat_items = flatten_nodes(nodes);
    
    // Calculate max key length (use icon as label)
    let max_key_len = flat_items.iter().map(|(_, _, icon)| strip_ansi_codes(icon).chars().count()).max().unwrap_or(0);
    // Calculate max val length
    let max_val_len = flat_items.iter().map(|(_, v, _)| strip_ansi_codes(v).chars().count()).max().unwrap_or(0);

    let left_width = max_key_len + 2;
    let right_width = max_val_len + 2;

    // Top borders
    // ╭───╮ ╭───╮
    let top = format!(
        "\x1b[38;5;2m╭{}╮\x1b[0m \x1b[38;5;2m╭{}╮\x1b[0m", 
        "─".repeat(left_width), 
        "─".repeat(right_width)
    );
    lines.push(top);

    for (key, val, icon) in flat_items {
        // Color key based on config or rainbow
        let color_code = get_color_code(&key, config);
        // Use icon as label text!
        let key_str = format!("\x1b[{}m{:<width$}\x1b[0m", color_code, icon, width = max_key_len);
        
        let val_stripped_len = strip_ansi_codes(&val).chars().count();
        let padding = max_val_len - val_stripped_len;

        let line = format!(
            "\x1b[38;5;2m│\x1b[0m {} \x1b[38;5;2m│\x1b[0m \x1b[38;5;2m│\x1b[0m {}{} \x1b[38;5;2m│\x1b[0m",
            key_str,
            val,
            " ".repeat(padding)
        );
        lines.push(line);
    }

    // Bottom borders
    let bottom = format!(
        "\x1b[38;5;2m╰{}╯\x1b[0m \x1b[38;5;2m╰{}╯\x1b[0m", 
        "─".repeat(left_width), 
        "─".repeat(right_width)
    );
    lines.push(bottom);

    lines
}

// Image 2: Tree
fn render_tree(nodes: &[RenderNode], config: &Config) -> Vec<String> {
    let mut lines = Vec::new();
    
    for node in nodes {
        match node {
            RenderNode::Group { title, children } => {
                // Root: [Icon] Title
                // Find icon for title if exists, or use default
                let icon = config.icons.get(title.to_lowercase().as_str()).map(|s| s.as_str()).unwrap_or(""); // Default PC icon
                let color_code = get_color_code(&title.to_lowercase(), config);
                
                lines.push(format!("\x1b[{}m{} {}\x1b[0m", color_code, icon, title));
                
                for (idx, child) in children.iter().enumerate() {
                    let is_last = idx == children.len() - 1;
                    let prefix = if is_last { "└──" } else { "├──" };
                    
                    if let RenderNode::Line { key, value, icon: _ } = child {
                        // Tree style: ├── Key Value
                         let key_color = get_color_code(key, config);
                         lines.push(format!(
                             "\x1b[38;5;240m{}\x1b[0m \x1b[{}m{}\x1b[0m {}", 
                             prefix, 
                             key_color, 
                             key, 
                             value
                         ));
                    }
                }
            },
            RenderNode::Line { key, value, icon } => {
                // Top level item
                 lines.push(format_line(key, value, icon, config));
            },
        }
    }
    lines
}

// Image 3: Section
fn render_section(nodes: &[RenderNode], config: &Config) -> Vec<String> {
    let mut lines = Vec::new();
    
    for node in nodes {
        match node {
            RenderNode::Group { title, children } => {
                // ┌─── Title ───┐ (Simplified)
                // ---- Title ----
                let header = format!("\x1b[38;5;240m──────\x1b[0m \x1b[1m{}\x1b[0m \x1b[38;5;240m──────\x1b[0m", title);
                lines.push(header);
                
                for child in children {
                     if let RenderNode::Line { key, value, icon } = child {
                         // Uses a special L-shape or just pipe?
                         // Image 3 uses: L: ...
                         // Image 4 uses tree style: ├──
                         // Let's use tree style
                         let _icon_display = if icon == "●" { "└" } else { icon }; // Use icon if specific, else tree
                         
                         let key_color = get_color_code(key, config);
                         lines.push(format!(
                             "\x1b[38;5;240m│\x1b[0m \x1b[{}m{} {}:\x1b[0m {}", 
                             key_color,
                             icon,
                             key, 
                             value
                         ));
                    }
                }
                lines.push("".to_string()); // Empty line
            },
             RenderNode::Line { key, value, icon } => {
                 lines.push(format_line(key, value, icon, config));
            },
        }
    }
    lines
}

// Helper to flatten nodes for classic layouts
fn flatten_nodes(nodes: &[RenderNode]) -> Vec<(String, String, String)> {
    let mut items = Vec::new();
    for node in nodes {
        match node {
            RenderNode::Line { key, value, icon } => items.push((key.clone(), value.clone(), icon.clone())),
            RenderNode::Group { children, .. } => {
                let mut child_items = flatten_nodes(children);
                items.append(&mut child_items);
            },
        }
    }
    items
}

fn format_line(key: &str, value: &str, icon: &str, config: &Config) -> String {
    let color_code = get_color_code(key, config);
    format!(
        "\x1b[{}m{} \x1b[0m{}", 
        color_code, 
        icon, 
        value
    )
}

fn format_line_content(key: &str, value: &str, icon: &str, config: &Config) -> String {
    let color_code = get_color_code(key, config);
    format!("\x1b[{}m{} \x1b[0m{}", color_code, icon, value)
}

fn get_color_code(key: &str, config: &Config) -> &'static str {
    let color_name = config.colors.get(key).map(|s| s.as_str()).unwrap_or("White");
    match color_name.to_lowercase().as_str() {
        "black" => "30",
        "red" => "31",
        "green" => "32",
        "yellow" => "33",
        "blue" => "34",
        "magenta" => "35",
        "cyan" => "36",
        "white" => "37",
        "grey" | "gray" => "90",
        _ => "37",
    }
}

fn format_palette(config: &Config) -> String {
    let style = config.palette_style.as_deref().unwrap_or("squares");
    let mut s = String::new();
    
    let colors = [40, 41, 42, 43, 44, 45, 46, 47]; // ANSI bg codes
    
    match style {
        "squares" => {
            for c in colors {
                s.push_str(&format!("\x1b[{}m  \x1b[0m ", c));
            }
        },
        _ => {
             for c in colors {
                s.push_str(&format!("\x1b[{}m  \x1b[0m ", c));
            }
        }
    }
    s
}

fn get_default_ascii() -> String {
    r#"
████████           ████████
 ████████         ████████
  ████████       ████████
   ████████     ████████
    ████████   ████████
     ████████ ████████
      ███████████████
       █████████████
      ███████████████
     ████████ ████████
    ████████   ████████
   ████████     ████████
  ████████       ████████
 ████████         ████████
████████           ████████
"#.trim().to_string()
}
