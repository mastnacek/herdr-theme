use crate::shared::client::HerdrClient;
use crate::shared::config::{default_herdr_config_path, read_current_theme, update_herdr_theme};
use crate::shared::terminal::emit_theme_skin;
use crate::shared::theme::{find_theme, counterpart_theme, ALL_THEMES, ThemeMode};
use std::io;

pub fn run_apply(theme_name: &str) -> io::Result<()> {
    let theme = match find_theme(theme_name) {
        Some(t) => t,
        None => {
            eprintln!("Unknown theme '{}'. Run 'herdr_theme-switcher list' to see available themes.", theme_name);
            return Err(io::Error::new(io::ErrorKind::NotFound, "Theme not found"));
        }
    };

    let config_path = default_herdr_config_path();
    update_herdr_theme(&config_path, theme)?;

    // Emit OSC 10/11 escape sequences to skin the host terminal / pane
    let _ = emit_theme_skin(theme);

    // Reload running Herdr server
    let client = HerdrClient::new();
    let reloaded = client.reload_server();

    let reload_status = if reloaded { "reloaded server" } else { "config saved" };
    println!("Applied theme '{}' ({}, {})", theme.name, theme.display_name, reload_status);
    client.notify("Theme Changed", &format!("Herdr theme set to {}", theme.display_name));

    Ok(())
}

pub fn run_toggle() -> io::Result<()> {
    let config_path = default_herdr_config_path();
    let current_name = read_current_theme(&config_path).unwrap_or_else(|| "catppuccin".to_string());
    let current_theme = find_theme(&current_name).unwrap_or_else(|| find_theme("catppuccin").unwrap());

    let target_theme = counterpart_theme(current_theme).unwrap_or_else(|| {
        if current_theme.mode == ThemeMode::Dark {
            find_theme("solarized-light").unwrap()
        } else {
            find_theme("catppuccin").unwrap()
        }
    });

    run_apply(target_theme.name)
}

pub fn run_list() -> io::Result<()> {
    let config_path = default_herdr_config_path();
    let current_name = read_current_theme(&config_path);

    println!("{:<20} {:<10} {:<10} {:<10} {:<12} {}", "NAME", "MODE", "BG", "FG", "SIDEBAR", "SLOT");
    println!("{}", "-".repeat(74));

    for theme in ALL_THEMES {
        let is_current = current_name.as_deref() == Some(theme.name);
        let marker = if is_current { "* " } else { "  " };
        let mode_str = match theme.mode {
            ThemeMode::Dark => "Dark",
            ThemeMode::Light => "Light",
        };
        let slot_str = theme.circadian_slot.unwrap_or("-");

        println!(
            "{}{:<18} {:<10} {:<10} {:<10} {:<12} {}",
            marker,
            theme.name,
            mode_str,
            theme.bg_hex,
            theme.fg_hex,
            theme.sidebar_bg,
            slot_str
        );
    }

    Ok(())
}

pub fn run_current() -> io::Result<()> {
    let config_path = default_herdr_config_path();
    let current_name = read_current_theme(&config_path).unwrap_or_else(|| "catppuccin (default)".to_string());
    println!("Current Herdr theme: {}", current_name);
    println!("Config path: {}", config_path.display());
    Ok(())
}
