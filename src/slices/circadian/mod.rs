use crate::shared::client::HerdrClient;
use crate::shared::config::{default_herdr_config_path, update_herdr_theme};
use crate::shared::solar::determine_slot;
use crate::shared::terminal::emit_theme_skin;
use crate::shared::theme::circadian_theme_for_slot;
use chrono::Utc;
use std::env;
use std::io;

pub fn run_circadian() -> io::Result<()> {
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
    update_herdr_theme(&config_path, theme)?;

    // Emit OSC 10/11 escape sequences to skin the host terminal / pane
    let _ = emit_theme_skin(theme);

    // Reload running Herdr server
    let client = HerdrClient::new();
    let reloaded = client.reload_server();

    let reload_status = if reloaded { "reloaded server" } else { "config saved" };
    println!(
        "Circadian sync: slot='{}' (elevation={:.1}°), applied '{}' ({}, {})",
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

    Ok(())
}
