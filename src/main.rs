mod config;
mod info;
mod ui;

use crate::config::load_config;
use crate::info::Info;
use crate::ui::draw;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to config file
    #[arg(short, long)]
    config: Option<String>,
}

fn main() {
    let args = Args::parse();
    
    // Load config
    let config = load_config(args.config);

    // Gather info
    let info = Info::new();

    // Draw
    draw(&info, &config);
}
