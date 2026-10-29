use crate::shared::client::HerdrClient;
use crate::shared::config::{default_herdr_config_path, read_current_theme, update_herdr_theme};
use crate::shared::solar::determine_slot;
use crate::shared::terminal::emit_theme_skin;
use crate::shared::theme::circadian_theme_for_slot;
use chrono::{Local, Utc};
use std::env;
use std::io;
use std::thread;
use std::time::Duration;

pub fn sync_circadian(force: bool) -> io::Result<bool> {
    let lat: f64 = env::var("LATITUDE")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(50.0755); // Prague default

    let lon: f64 = env::var("LONGITUDE")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(14.4378); // Prague default

    let now = Utc::now();
    let (slot, elevation) = determine_slot(now, lat, lon);
    let theme = circadian_theme_for_slot(slot.as_str());

    let config_path = default_herdr_config_path();
    let current_active = read_current_theme(&config_path);

    // If theme is already active and we are not forcing, skip disk write and server reload
    if !force && current_active.as_deref() == Some(theme.name) {
        return Ok(false);
    }

    update_herdr_theme(&config_path, theme)?;

    // Emit OSC 10/11 escape sequences to skin the host terminal / pane
    let _ = emit_theme_skin(theme);

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
    client.notify(
        "Circadian Theme Sync",
        &format!("Phase: {} -> {}", slot.as_str(), theme.display_name),
    );

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
    let lat: f64 = env::var("LATITUDE")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(50.0755);
    let lon: f64 = env::var("LONGITUDE")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(14.4378);

    println!("=====================================================");
    println!(" Herdr Circadian Theme Watcher Daemon");
    println!(" Coordinates: {:.4}°N, {:.4}°E", lat, lon);
    println!(" Check Interval: {}s", interval_secs);
    println!(" Press Ctrl+C to stop.");
    println!("=====================================================");

    // Perform initial sync on startup
    let _ = sync_circadian(false);

    loop {
        thread::sleep(Duration::from_secs(interval_secs));
        if let Err(e) = sync_circadian(false) {
            eprintln!("[{}] Error syncing circadian theme: {}", Local::now().format("%H:%M:%S"), e);
        }
    }
}
