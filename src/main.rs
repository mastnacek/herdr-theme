mod shared;
mod slices;

use std::env;
use std::io;

fn print_usage() {
    println!("Herdr Theme Switcher");
    println!("Usage:");
    println!("  herdr_theme-switcher [picker]               Open interactive theme manager TUI (Themes tab)");
    println!("  herdr_theme-switcher settings [--tui|--path] Display or configure settings (interactive TUI form)");
    println!("  herdr_theme-switcher settings-tui            Open interactive settings configuration form directly");
    println!("  herdr_theme-switcher apply <theme>           Apply specific theme directly");
    println!("  herdr_theme-switcher toggle                  Toggle between light and dark counterpart");
    println!("  herdr_theme-switcher circadian [--force]     Sync theme to current solar elevation");
    println!("  herdr_theme-switcher watch [--interval <sec>] Background daemon monitoring solar elevation (foreground)");
    println!("  herdr_theme-switcher start-daemon            Launch background watcher daemon (detached)");
    println!("  herdr_theme-switcher stop-daemon             Stop running background watcher daemon");
    println!("  herdr_theme-switcher status-daemon           Check background watcher daemon status");
    println!("  herdr_theme-switcher settings [--path]       Display current plugin settings and config path");
    println!("  herdr_theme-switcher list                    List all supported themes with hex colors");
    println!("  herdr_theme-switcher current                 Display current theme from config");
    println!("  herdr_theme-switcher reset-skin              Reset terminal background/text colors");
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let command = args.get(1).map(|s| s.as_str()).unwrap_or("picker");

    match command {
        "picker" | "popup" => slices::picker::run_popup(slices::picker::state::ActiveTab::Themes)?,
        "open-popup" => {
            let entrypoint = if cfg!(windows) { "picker-win" } else { "picker" };
            let client = shared::client::HerdrClient::new();
            let _ = std::process::Command::new(&client.bin)
                .args(["plugin", "pane", "open", "--plugin", "herdr.theme-switcher", "--entrypoint", entrypoint])
                .status();
        }
        "open-settings" => {
            let entrypoint = if cfg!(windows) { "settings-win" } else { "settings" };
            let client = shared::client::HerdrClient::new();
            let _ = std::process::Command::new(&client.bin)
                .args(["plugin", "pane", "open", "--plugin", "herdr.theme-switcher", "--entrypoint", entrypoint])
                .status();
        }
        "settings-tui" | "config-tui" => slices::picker::run_popup(slices::picker::state::ActiveTab::Settings)?,
        "solar-tui" => slices::picker::run_popup(slices::picker::state::ActiveTab::Solar)?,
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
        "circadian" => {
            let force = args.iter().any(|a| a == "--force" || a == "-f");
            slices::circadian::run_circadian(force)?;
        }
        "watch" | "daemon" => {
            let mut interval = 60;
            if let Some(pos) = args.iter().position(|a| a == "--interval" || a == "-i") {
                if let Some(val) = args.get(pos + 1) {
                    if let Ok(num) = val.parse::<u64>() {
                        interval = num;
                    }
                }
            }
            slices::circadian::run_watch(interval)?;
        }
        "start-daemon" => {
            let mut interval = 60;
            if let Some(pos) = args.iter().position(|a| a == "--interval" || a == "-i") {
                if let Some(val) = args.get(pos + 1) {
                    if let Ok(num) = val.parse::<u64>() {
                        interval = num;
                    }
                }
            }
            shared::daemon::start_daemon(interval)?;
        }
        "stop-daemon" => {
            shared::daemon::stop_daemon()?;
        }
        "status-daemon" | "daemon-status" => {
            shared::daemon::status_daemon()?;
        }
        "list" => slices::apply::run_list()?,
        "current" => slices::apply::run_current()?,
        "settings" | "config" => {
            if args.iter().any(|a| a == "--tui" || a == "-t") {
                slices::picker::run_popup(slices::picker::state::ActiveTab::Settings)?;
                return Ok(());
            }
            let path = shared::PluginSettings::settings_path();
            if args.iter().any(|a| a == "--path" || a == "-p") {
                println!("{}", path.display());
            } else {
                let s = shared::PluginSettings::load();
                println!("Herdr Theme Switcher Settings");
                println!("File: {}", path.display());
                println!("-----------------------------------------------------");
                println!("Location:       {:.4}°N, {:.4}°E", s.location.latitude, s.location.longitude);
                println!("Daemon Watch:   every {}s", s.daemon.interval_seconds);
                println!("Circadian Slot Themes:");
                println!("  Dawn:         {}", s.circadian.dawn);
                println!("  Day:          {}", s.circadian.day);
                println!("  Dusk:         {}", s.circadian.dusk);
                println!("  Night:        {}", s.circadian.night);
                println!("Terminal Features:");
                println!("  Skin (OSC):   {}", if s.terminal.skin_terminal { "enabled" } else { "disabled" });
                println!("  Notify:       {}", if s.terminal.notify { "enabled" } else { "disabled" });
            }
        }
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
