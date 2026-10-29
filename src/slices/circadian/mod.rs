use crate::shared::client::HerdrClient;
use crate::shared::config::{default_herdr_config_path, read_current_theme, update_herdr_theme};
use crate::shared::solar::determine_slot;
use crate::shared::terminal::emit_theme_skin;
use crate::shared::theme::{circadian_theme_for_slot, find_theme};
use crate::shared::PluginSettings;
use chrono::{Local, Utc};
use std::io;
use std::thread;
use std::time::Duration;

pub fn sync_circadian(force: bool) -> io::Result<bool> {
    let settings = PluginSettings::load();
    let lat = settings.location.latitude;
    let lon = settings.location.longitude;

    let now = Utc::now();
    let (slot, elevation) = determine_slot(now, lat, lon);
    let theme_name = settings.theme_for_phase(slot);
    let theme = find_theme(theme_name).unwrap_or_else(|| {
        circadian_theme_for_slot(slot.as_str())
    });

    let config_path = default_herdr_config_path();
    let current_active = read_current_theme(&config_path);

    // If theme is already active and we are not forcing, skip disk write and server reload
    if !force && current_active.as_deref() == Some(theme.name) {
        return Ok(false);
    }

    update_herdr_theme(&config_path, theme)?;

    // Emit OSC 10/11 escape sequences to skin the host terminal / pane if enabled
    if settings.terminal.skin_terminal {
        let _ = emit_theme_skin(theme);
    }

    // Reload running Herdr server
    let client = HerdrClient::new();
    let reloaded = client.reload_server();

    let reload_status = if reloaded { "reloaded server" } else { "config saved" };
    let time_str = Local::now().format("%H:%M:%S");

    println!(
        "[{}] Circadian transition: slot='{}' (elevation={:.1}°), applied '{}' ({}, {})",
        time_str,
        slot.as_str(),
        elevation,
        theme.name,
        theme.display_name,
        reload_status
    );

    if settings.terminal.notify {
        client.notify(
            "Circadian Theme Sync",
            &format!("Phase: {} -> {}", slot.as_str(), theme.display_name),
        );
    }

    Ok(true)
}

pub fn run_circadian(force: bool) -> io::Result<()> {
    let changed = sync_circadian(force)?;
    if !changed {
        println!("Circadian theme already in sync with solar phase.");
    }
    Ok(())
}

pub fn run_watch(interval_secs: u64) -> io::Result<()> {
    let settings = PluginSettings::load();
    let effective_interval = if interval_secs == 60 {
        settings.daemon.interval_seconds
    } else {
        interval_secs
    };

    println!("=====================================================");
    println!(" Herdr Circadian Theme Watcher Daemon");
    println!(" Coordinates: {:.4}°N, {:.4}°E", settings.location.latitude, settings.location.longitude);
    println!(" Check Interval: {}s", effective_interval);
    println!(" Config File: {}", PluginSettings::settings_path().display());
    println!(" Press Ctrl+C to stop.");
    println!("=====================================================");

    // Perform initial sync on startup
    let _ = sync_circadian(false);

    loop {
        thread::sleep(Duration::from_secs(effective_interval));
        if let Err(e) = sync_circadian(false) {
            eprintln!("[{}] Error syncing circadian theme: {}", Local::now().format("%H:%M:%S"), e);
        }
    }
}
