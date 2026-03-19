mod config;
mod info;
mod ui;

use crate::config::{generate_config, load_config};
use crate::info::Info;
use crate::ui::draw;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about,
    long_about = None,
    after_help = "Examples:\n  xfetch\n  xfetch --config ~/.config/xfetch/config.jsonc\n  xfetch --gen-config"
)]
struct Args {
    /// Path to config file
    #[arg(short, long)]
    config: Option<String>,

    /// Generate a default config.jsonc (pacman layout) and exit
    #[arg(long)]
    gen_config: bool,
}

fn main() {
    let args = Args::parse();

    if args.gen_config {
        match generate_config(args.config.clone()) {
            Ok(path) => {
                println!("Generated config: {}", path.display());
                println!("Run xfetch to see the new layout.");
                return;
            }
            Err(err) => {
                eprintln!("Failed to generate config: {}", err);
                std::process::exit(1);
            }
        }
    }
    
    // Load config
    let config = load_config(args.config);

    // Gather info
    let info = Info::new();

    // Draw
    draw(&info, &config);
}
