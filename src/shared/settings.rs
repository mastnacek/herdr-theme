use crate::shared::solar::SolarSlot;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationSettings {
    #[serde(default = "default_latitude")]
    pub latitude: f64,
    #[serde(default = "default_longitude")]
    pub longitude: f64,
}

fn default_latitude() -> f64 {
    50.0755
}

fn default_longitude() -> f64 {
    14.4378
}

impl Default for LocationSettings {
    fn default() -> Self {
        Self {
            latitude: default_latitude(),
            longitude: default_longitude(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircadianSettings {
    #[serde(default = "default_dawn")]
    pub dawn: String,
    #[serde(default = "default_day")]
    pub day: String,
    #[serde(default = "default_dusk")]
    pub dusk: String,
    #[serde(default = "default_night")]
    pub night: String,
}

fn default_dawn() -> String {
    "gruvbox".to_string()
}
fn default_day() -> String {
    "solarized-light".to_string()
}
fn default_dusk() -> String {
    "catppuccin".to_string()
}
fn default_night() -> String {
    "osaka-jade".to_string()
}

impl Default for CircadianSettings {
    fn default() -> Self {
        Self {
            dawn: default_dawn(),
            day: default_day(),
            dusk: default_dusk(),
            night: default_night(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaemonSettings {
    #[serde(default = "default_interval")]
    pub interval_seconds: u64,
}

fn default_interval() -> u64 {
    60
}

impl Default for DaemonSettings {
    fn default() -> Self {
        Self {
            interval_seconds: default_interval(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalSettings {
    #[serde(default = "default_true")]
    pub skin_terminal: bool,
    #[serde(default = "default_true")]
    pub notify: bool,
}

fn default_true() -> bool {
    true
}

impl Default for TerminalSettings {
    fn default() -> Self {
        Self {
            skin_terminal: true,
            notify: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PluginSettings {
    #[serde(default)]
    pub location: LocationSettings,
    #[serde(default)]
    pub circadian: CircadianSettings,
    #[serde(default)]
    pub daemon: DaemonSettings,
    #[serde(default)]
    pub terminal: TerminalSettings,
}

impl PluginSettings {
    /// Resolves the plugin settings file path inside HERDR_PLUGIN_CONFIG_DIR.
    pub fn settings_path() -> PathBuf {
        if let Ok(dir) = env::var("HERDR_PLUGIN_CONFIG_DIR") {
            let p = PathBuf::from(dir);
            let _ = fs::create_dir_all(&p);
            return p.join("settings.toml");
        }

        #[cfg(windows)]
        {
            if let Ok(appdata) = env::var("APPDATA") {
                let p = PathBuf::from(appdata)
                    .join("herdr")
                    .join("plugins")
                    .join("config")
                    .join("herdr.theme-switcher");
                let _ = fs::create_dir_all(&p);
                return p.join("settings.toml");
            }
            PathBuf::from(r"C:\Users\Default\AppData\Roaming\herdr\plugins\config\herdr.theme-switcher\settings.toml")
        }

        #[cfg(not(windows))]
        {
            if let Ok(home) = env::var("HOME") {
                let p = PathBuf::from(home)
                    .join(".config")
                    .join("herdr")
                    .join("plugins")
                    .join("config")
                    .join("herdr.theme-switcher");
                let _ = fs::create_dir_all(&p);
                return p.join("settings.toml");
            }
            PathBuf::from("/tmp/herdr.theme-switcher-settings.toml")
        }
    }

    /// Loads settings from disk or creates default template if absent.
    pub fn load() -> Self {
        let path = Self::settings_path();
        let mut settings = if path.exists() {
            if let Ok(content) = fs::read_to_string(&path) {
                toml::from_str::<PluginSettings>(&content).unwrap_or_default()
            } else {
                PluginSettings::default()
            }
        } else {
            let s = PluginSettings::default();
            let _ = s.save();
            s
        };

        // Environment overrides have highest precedence
        if let Ok(lat) = env::var("LATITUDE") {
            if let Ok(val) = lat.parse::<f64>() {
                settings.location.latitude = val;
            }
        }
        if let Ok(lon) = env::var("LONGITUDE") {
            if let Ok(val) = lon.parse::<f64>() {
                settings.location.longitude = val;
            }
        }

        settings
    }

    /// Saves the settings to disk with comments.
    pub fn save(&self) -> std::io::Result<()> {
        let path = Self::settings_path();
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }

        let content = format!(
            r#"# Herdr Theme Switcher Configuration
# Automatically reloaded by the theme switcher and daemon.

[location]
# Latitude and Longitude for NOAA astronomical solar calculation
# Default: Prague, Czech Republic (50.0755, 14.4378)
latitude = {latitude:.4}
longitude = {longitude:.4}

[circadian]
# Theme assigned to each solar phase
# Use 'herdr_theme-switcher list' to see all 18 available themes
dawn = "{dawn}"           # -12° to -0.83° (rising sun)
day = "{day}"             # > -0.83° (sun above horizon)
dusk = "{dusk}"           # -0.83° to -12° (setting sun)
night = "{night}"         # < -12° (astronomical night)

[daemon]
# Background watcher cycle in seconds (default: 60)
interval_seconds = {interval}

[terminal]
# Automatically skin host terminal background & foreground via OSC 10/11
skin_terminal = {skin}

# Send desktop notification on automatic theme transition
notify = {notify}
"#,
            latitude = self.location.latitude,
            longitude = self.location.longitude,
            dawn = self.circadian.dawn,
            day = self.circadian.day,
            dusk = self.circadian.dusk,
            night = self.circadian.night,
            interval = self.daemon.interval_seconds,
            skin = self.terminal.skin_terminal,
            notify = self.terminal.notify
        );

        fs::write(path, content)
    }

    /// Returns the theme name configured for a specific solar phase/slot.
    pub fn theme_for_phase(&self, slot: SolarSlot) -> &str {
        match slot {
            SolarSlot::Dawn => &self.circadian.dawn,
            SolarSlot::Day => &self.circadian.day,
            SolarSlot::Dusk => &self.circadian.dusk,
            SolarSlot::Night => &self.circadian.night,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_settings() {
        let s = PluginSettings::default();
        assert!((s.location.latitude - 50.0755).abs() < 0.001);
        assert!((s.location.longitude - 14.4378).abs() < 0.001);
        assert_eq!(s.circadian.day, "solarized-light");
        assert_eq!(s.circadian.night, "osaka-jade");
        assert_eq!(s.theme_for_phase(SolarSlot::Day), "solarized-light");
        assert_eq!(s.theme_for_phase(SolarSlot::Night), "osaka-jade");
        assert_eq!(s.daemon.interval_seconds, 60);
        assert!(s.terminal.skin_terminal);
        assert!(s.terminal.notify);
    }

    #[test]
    fn test_toml_deserialization() {
        let toml_str = r#"
[location]
latitude = 40.7128
longitude = -74.0060

[circadian]
day = "catppuccin-latte"
night = "tokyo-night"

[daemon]
interval_seconds = 30
"#;
        let s: PluginSettings = toml::from_str(toml_str).unwrap();
        assert!((s.location.latitude - 40.7128).abs() < 0.001);
        assert_eq!(s.circadian.day, "catppuccin-latte");
        assert_eq!(s.circadian.night, "tokyo-night");
        assert_eq!(s.circadian.dawn, "gruvbox"); // default fallback
        assert_eq!(s.daemon.interval_seconds, 30);
    }
}
