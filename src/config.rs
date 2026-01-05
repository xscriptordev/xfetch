use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use json_comments::StripComments;
use std::io::Read;

use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct Config {
    pub ascii: Option<String>,
    pub logo_path: Option<String>,
    pub modules: Vec<String>,
    pub show_colors: bool,
    pub icons: HashMap<String, String>,
    pub colors: HashMap<String, String>,
    pub layout: Option<String>,
    pub header_icons: Option<Vec<String>>,
    pub footer_text: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        let mut icons = HashMap::new();
        icons.insert("os".to_string(), "".to_string()); // Arch icon default
        icons.insert("kernel".to_string(), "".to_string());
        icons.insert("hostname".to_string(), "".to_string()); // Hostname icon
        icons.insert("wm".to_string(), "".to_string());
        icons.insert("packages".to_string(), "".to_string());
        icons.insert("shell".to_string(), "".to_string());
        icons.insert("cpu".to_string(), "".to_string());
        icons.insert("gpu".to_string(), "󰍹".to_string());
        icons.insert("memory".to_string(), "".to_string());
        icons.insert("disk".to_string(), "󰋊".to_string());
        icons.insert("battery".to_string(), "".to_string());
        icons.insert("uptime".to_string(), "󰔛".to_string());
        icons.insert("terminal".to_string(), "".to_string());

        let mut colors = HashMap::new();
        // Default colors (using crossterm color names as strings)
        colors.insert("os".to_string(), "Cyan".to_string());
        colors.insert("kernel".to_string(), "White".to_string());
        colors.insert("wm".to_string(), "Blue".to_string());
        
        Self {
            ascii: None,
            logo_path: None,
            modules: vec![
                "os".to_string(),
                "kernel".to_string(),
                "uptime".to_string(),
                "packages".to_string(),
                "wm".to_string(),
                "shell".to_string(),
                "disk".to_string(),
                "cpu".to_string(),
                "gpu".to_string(),
                "memory".to_string(),
                "battery".to_string(),
            ],
            show_colors: true,
            icons,
            colors,
            layout: None,
            header_icons: None,
            footer_text: None,
        }
    }
}

pub fn load_config(path: Option<String>) -> Config {
    let config_path = if let Some(p) = path {
        PathBuf::from(p)
    } else {
        let config_dir = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        config_dir.join("xfetch").join("config.jsonc")
    };
    
    // println!("Debug: Config path is {:?}", config_path);

    if !config_path.exists() {
        // println!("Debug: Config file does not exist");
        return Config::default();
    }

    let file = match fs::File::open(&config_path) {
        Ok(f) => f,
        Err(_) => return Config::default(),
    };

    let mut stripped = StripComments::new(file);
    let mut content = String::new();
    if stripped.read_to_string(&mut content).is_err() {
        return Config::default();
    }
    
    // println!("Debug: Config content: {}", content);

    match serde_json::from_str(&content) {
        Ok(c) => c,
        Err(_e) => {
            // println!("Debug: Failed to parse config: {}", e);
            Config::default()
        }
    }
}
