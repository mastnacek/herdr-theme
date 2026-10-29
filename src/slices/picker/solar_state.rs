use crate::shared::settings::PluginSettings;
use crate::shared::solar::{determine_slot, SolarSlot};
use chrono::{Local, Utc};

pub struct SolarState {
    pub elevation: f64,
    pub slot: SolarSlot,
    pub lat: f64,
    pub lon: f64,
    pub active_theme: String,
    pub utc_time: String,
    pub local_time: String,
}

impl SolarState {
    pub fn refresh() -> Self {
        let settings = PluginSettings::load();
        let lat = settings.location.latitude;
        let lon = settings.location.longitude;
        let now = Utc::now();
        let (slot, elevation) = determine_slot(now, lat, lon);
        let theme_name = settings.theme_for_phase(slot).to_string();

        Self {
            elevation,
            slot,
            lat,
            lon,
            active_theme: theme_name,
            utc_time: now.format("%Y-%m-%d %H:%M:%S UTC").to_string(),
            local_time: Local::now().format("%Y-%m-%d %H:%M:%S %Z").to_string(),
        }
    }
}
