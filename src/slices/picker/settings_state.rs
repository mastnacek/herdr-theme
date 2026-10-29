use crate::shared::daemon::{daemon_pid_path, is_pid_running, kill_pid, start_daemon};
use crate::shared::settings::PluginSettings;
use crate::shared::theme::ALL_THEMES;
use std::fs;

#[derive(Debug, Clone, Copy)]
pub struct CityPreset {
    pub name: &'static str,
    pub lat: f64,
    pub lon: f64,
}

pub const PRESET_CITIES: &[CityPreset] = &[
    CityPreset { name: "Prague, CZ", lat: 50.0755, lon: 14.4378 },
    CityPreset { name: "London, UK", lat: 51.5074, lon: -0.1278 },
    CityPreset { name: "Berlin, DE", lat: 52.5200, lon: 13.4050 },
    CityPreset { name: "Paris, FR", lat: 48.8566, lon: 2.3522 },
    CityPreset { name: "New York, US", lat: 40.7128, lon: -74.0060 },
    CityPreset { name: "San Francisco, US", lat: 37.7749, lon: -122.4194 },
    CityPreset { name: "Tokyo, JP", lat: 35.6762, lon: 139.6503 },
    CityPreset { name: "Sydney, AU", lat: -33.8688, lon: 151.2093 },
    CityPreset { name: "Custom Coordinates", lat: 0.0, lon: 0.0 },
];

pub const INTERVAL_OPTIONS: &[u64] = &[10, 30, 60, 120, 300, 600];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettingsField {
    DawnTheme = 0,
    DayTheme = 1,
    DuskTheme = 2,
    NightTheme = 3,
    CityPreset = 4,
    Latitude = 5,
    Longitude = 6,
    WatchInterval = 7,
    SkinTerminal = 8,
    DesktopNotify = 9,
    DaemonControl = 10,
    SaveButton = 11,
    ResyncButton = 12,
}

impl SettingsField {
    pub const TOTAL_FIELDS: usize = 13;

    pub fn from_index(idx: usize) -> Self {
        match idx {
            0 => SettingsField::DawnTheme,
            1 => SettingsField::DayTheme,
            2 => SettingsField::DuskTheme,
            3 => SettingsField::NightTheme,
            4 => SettingsField::CityPreset,
            5 => SettingsField::Latitude,
            6 => SettingsField::Longitude,
            7 => SettingsField::WatchInterval,
            8 => SettingsField::SkinTerminal,
            9 => SettingsField::DesktopNotify,
            10 => SettingsField::DaemonControl,
            11 => SettingsField::SaveButton,
            _ => SettingsField::ResyncButton,
        }
    }
}

pub struct SettingsState {
    pub settings: PluginSettings,
    pub focused_field: usize,
    pub selected_city_idx: usize,
    pub daemon_running: bool,
    pub daemon_pid: Option<u32>,
    pub status_message: Option<(String, bool)>, // (msg, is_error)
    pub is_dirty: bool,
}

impl SettingsState {
    pub fn new() -> Self {
        let settings = PluginSettings::load();
        let city_idx = Self::match_city_preset(settings.location.latitude, settings.location.longitude);
        let (daemon_running, daemon_pid) = Self::check_daemon();

        Self {
            settings,
            focused_field: 0,
            selected_city_idx: city_idx,
            daemon_running,
            daemon_pid,
            status_message: None,
            is_dirty: false,
        }
    }

    pub fn check_daemon() -> (bool, Option<u32>) {
        let pid_file = daemon_pid_path();
        if let Ok(content) = fs::read_to_string(&pid_file) {
            if let Ok(pid) = content.trim().parse::<u32>() {
                if is_pid_running(pid) {
                    return (true, Some(pid));
                }
            }
        }
        (false, None)
    }

    fn match_city_preset(lat: f64, lon: f64) -> usize {
        for (i, city) in PRESET_CITIES.iter().enumerate().take(PRESET_CITIES.len() - 1) {
            if (city.lat - lat).abs() < 0.01 && (city.lon - lon).abs() < 0.01 {
                return i;
            }
        }
        PRESET_CITIES.len() - 1 // Custom
    }

    pub fn next_field(&mut self) {
        self.focused_field = (self.focused_field + 1) % SettingsField::TOTAL_FIELDS;
    }

    pub fn prev_field(&mut self) {
        if self.focused_field == 0 {
            self.focused_field = SettingsField::TOTAL_FIELDS - 1;
        } else {
            self.focused_field -= 1;
        }
    }

    fn cycle_theme_name(current: &str, forward: bool) -> String {
        let idx = ALL_THEMES.iter().position(|t| t.name == current).unwrap_or(0);
        let next_idx = if forward {
            (idx + 1) % ALL_THEMES.len()
        } else if idx == 0 {
            ALL_THEMES.len() - 1
        } else {
            idx - 1
        };
        ALL_THEMES[next_idx].name.to_string()
    }

    pub fn adjust_field(&mut self, forward: bool) {
        self.status_message = None;
        let field = SettingsField::from_index(self.focused_field);

        match field {
            SettingsField::DawnTheme => {
                self.settings.circadian.dawn = Self::cycle_theme_name(&self.settings.circadian.dawn, forward);
                self.is_dirty = true;
            }
            SettingsField::DayTheme => {
                self.settings.circadian.day = Self::cycle_theme_name(&self.settings.circadian.day, forward);
                self.is_dirty = true;
            }
            SettingsField::DuskTheme => {
                self.settings.circadian.dusk = Self::cycle_theme_name(&self.settings.circadian.dusk, forward);
                self.is_dirty = true;
            }
            SettingsField::NightTheme => {
                self.settings.circadian.night = Self::cycle_theme_name(&self.settings.circadian.night, forward);
                self.is_dirty = true;
            }
            SettingsField::CityPreset => {
                let len = PRESET_CITIES.len();
                if forward {
                    self.selected_city_idx = (self.selected_city_idx + 1) % len;
                } else if self.selected_city_idx == 0 {
                    self.selected_city_idx = len - 1;
                } else {
                    self.selected_city_idx -= 1;
                }
                let preset = PRESET_CITIES[self.selected_city_idx];
                if self.selected_city_idx < len - 1 {
                    self.settings.location.latitude = preset.lat;
                    self.settings.location.longitude = preset.lon;
                }
                self.is_dirty = true;
            }
            SettingsField::Latitude => {
                let delta = if forward { 0.5 } else { -0.5 };
                self.settings.location.latitude = (self.settings.location.latitude + delta).clamp(-90.0, 90.0);
                self.selected_city_idx = PRESET_CITIES.len() - 1; // Custom
                self.is_dirty = true;
            }
            SettingsField::Longitude => {
                let delta = if forward { 0.5 } else { -0.5 };
                self.settings.location.longitude = (self.settings.location.longitude + delta).clamp(-180.0, 180.0);
                self.selected_city_idx = PRESET_CITIES.len() - 1; // Custom
                self.is_dirty = true;
            }
            SettingsField::WatchInterval => {
                let cur = self.settings.daemon.interval_seconds;
                let cur_idx = INTERVAL_OPTIONS.iter().position(|&v| v == cur).unwrap_or(2);
                let next_idx = if forward {
                    (cur_idx + 1) % INTERVAL_OPTIONS.len()
                } else if cur_idx == 0 {
                    INTERVAL_OPTIONS.len() - 1
                } else {
                    cur_idx - 1
                };
                self.settings.daemon.interval_seconds = INTERVAL_OPTIONS[next_idx];
                self.is_dirty = true;
            }
            SettingsField::SkinTerminal => {
                self.settings.terminal.skin_terminal = !self.settings.terminal.skin_terminal;
                self.is_dirty = true;
            }
            SettingsField::DesktopNotify => {
                self.settings.terminal.notify = !self.settings.terminal.notify;
                self.is_dirty = true;
            }
            SettingsField::DaemonControl => {
                self.toggle_daemon();
            }
            SettingsField::SaveButton => {
                self.save();
            }
            SettingsField::ResyncButton => {
                self.resync_now();
            }
        }
    }

    pub fn toggle_daemon(&mut self) {
        if self.daemon_running {
            if let Some(pid) = self.daemon_pid {
                let _ = kill_pid(pid);
                let _ = fs::remove_file(daemon_pid_path());
                self.daemon_running = false;
                self.daemon_pid = None;
                self.status_message = Some(("Daemon stopped.".to_string(), false));
            }
        } else {
            let interval = self.settings.daemon.interval_seconds;
            if let Ok(()) = start_daemon(interval) {
                let (running, pid) = Self::check_daemon();
                self.daemon_running = running;
                self.daemon_pid = pid;
                self.status_message = Some((format!("Daemon started (PID: {:?}).", pid), false));
            } else {
                self.status_message = Some(("Failed to start daemon.".to_string(), true));
            }
        }
    }

    pub fn save(&mut self) {
        match self.settings.save() {
            Ok(()) => {
                self.is_dirty = false;
                self.status_message = Some(("✔ Settings saved to settings.toml".to_string(), false));
            }
            Err(e) => {
                self.status_message = Some((format!("Error saving settings: {}", e), true));
            }
        }
    }

    pub fn resync_now(&mut self) {
        let _ = self.settings.save();
        self.is_dirty = false;
        match crate::slices::circadian::sync_circadian(true) {
            Ok(_) => {
                self.status_message = Some(("✔ Circadian theme synced successfully.".to_string(), false));
            }
            Err(e) => {
                self.status_message = Some((format!("Error syncing theme: {}", e), true));
            }
        }
    }
}
