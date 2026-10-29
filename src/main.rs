mod shared;
mod slices;

use std::env;
use std::io;

fn print_usage() {
    println!("Herdr Theme Switcher");
    println!("Usage:");
    println!("  herdr_theme-switcher [picker]       Open interactive theme picker popup");
    println!("  herdr_theme-switcher apply <theme>   Apply specific theme directly");
    println!("  herdr_theme-switcher toggle          Toggle between light and dark counterpart");
    println!("  herdr_theme-switcher circadian       Sync theme to current solar elevation");
    println!("  herdr_theme-switcher list            List all supported themes with hex colors");
    println!("  herdr_theme-switcher current         Display current theme from config");
    println!("  herdr_theme-switcher reset-skin      Reset terminal background/text colors");
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let command = args.get(1).map(|s| s.as_str()).unwrap_or("picker");

    match command {
        "picker" | "popup" => slices::picker::run_popup()?,
        "apply" | "set" => {
            if let Some(theme_name) = args.get(2) {
                slices::apply::run_apply(theme_name)?;
            } else {
                eprintln!("Error: 'apply' requires a theme name argument.");
                print_usage();
                std::process::exit(1);
            }
        }
        "toggle" => slices::apply::run_toggle()?,
        "circadian" => slices::circadian::run_circadian()?,
        "list" => slices::apply::run_list()?,
        "current" => slices::apply::run_current()?,
        "reset-skin" => {
            shared::terminal::reset_terminal_skin()?;
            println!("Terminal skin reset to profile defaults.");
        }
        "-h" | "--help" | "help" => {
            print_usage();
        }
        unknown => {
            eprintln!("Unknown command: '{}'\n", unknown);
            print_usage();
            std::process::exit(1);
        }
    }

    Ok(())
}
