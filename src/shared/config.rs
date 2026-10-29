use crate::shared::theme::ThemeInfo;
use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use toml_edit::{table, value, DocumentMut};

pub fn default_herdr_config_path() -> PathBuf {
    if let Ok(p) = env::var("HERDR_CONFIG_PATH") {
        let trimmed = p.trim();
        if !trimmed.is_empty() {
            return PathBuf::from(trimmed);
        }
    }

    #[cfg(windows)]
    {
        if let Ok(appdata) = env::var("APPDATA") {
            return PathBuf::from(appdata).join("herdr").join("config.toml");
        }
        if let Ok(userprofile) = env::var("USERPROFILE") {
            return PathBuf::from(userprofile)
                .join("AppData")
                .join("Roaming")
                .join("herdr")
                .join("config.toml");
        }
        PathBuf::from(r"C:\Users\Default\AppData\Roaming\herdr\config.toml")
    }

    #[cfg(not(windows))]
    {
        if let Ok(xdg) = env::var("XDG_CONFIG_HOME") {
            return PathBuf::from(xdg).join("herdr").join("config.toml");
        }
        if let Ok(home) = env::var("HOME") {
            return PathBuf::from(home).join(".config").join("herdr").join("config.toml");
        }
        PathBuf::from("~/.config/herdr/config.toml")
    }
}

pub fn read_current_theme(path: &Path) -> Option<String> {
    if !path.exists() {
        return None;
    }
    let content = fs::read_to_string(path).ok()?;
    let doc: DocumentMut = content.parse().ok()?;
    let theme_table = doc.get("theme")?;
    let name = theme_table.get("name")?.as_str()?;
    Some(name.to_string())
}

pub fn update_herdr_theme(path: &Path, theme: &ThemeInfo) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let mut doc: DocumentMut = if path.exists() {
        let content = fs::read_to_string(path)?;
        content.parse().map_err(|e| {
            io::Error::new(io::ErrorKind::InvalidData, format!("Failed to parse TOML: {}", e))
        })?
    } else {
        DocumentMut::new()
    };

    // Ensure [theme] table exists
    if !doc.contains_key("theme") || !doc["theme"].is_table_like() {
        doc["theme"] = table();
    }

    // Set theme.name
    doc["theme"]["name"] = value(theme.name);

    // Disable auto_switch so Herdr honors the chosen theme
    doc["theme"]["auto_switch"] = value(false);

    // Ensure [theme.custom] table exists
    if !doc["theme"].as_table_like().unwrap().contains_key("custom")
        || !doc["theme"]["custom"].is_table_like()
    {
        doc["theme"]["custom"] = table();
    }

    // Set custom.sidebar_bg
    doc["theme"]["custom"]["sidebar_bg"] = value(theme.sidebar_bg);

    // Write out atomically using a temporary file in the same directory
    let toml_str = doc.to_string();
    let tmp_path = path.with_extension("tmp");
    fs::write(&tmp_path, toml_str)?;
    fs::rename(&tmp_path, path)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared::theme::find_theme;

    #[test]
    fn test_update_empty_config() {
        let tmp_dir = env::temp_dir().join("herdr_theme_test_empty");
        let _ = fs::remove_dir_all(&tmp_dir);
        let config_path = tmp_dir.join("config.toml");

        let theme = find_theme("solarized-light").unwrap();
        update_herdr_theme(&config_path, theme).unwrap();

        assert!(config_path.exists());
        let current = read_current_theme(&config_path);
        assert_eq!(current, Some("solarized-light".to_string()));

        let _ = fs::remove_dir_all(&tmp_dir);
    }

    #[test]
    fn test_preserve_existing_keys_and_comments() {
        let tmp_dir = env::temp_dir().join("herdr_theme_test_preserve");
        let _ = fs::remove_dir_all(&tmp_dir);
        fs::create_dir_all(&tmp_dir).unwrap();
        let config_path = tmp_dir.join("config.toml");

        let initial = r##"# Initial comment
[keys]
prefix = "ctrl+b"

[theme]
# theme name comment
name = "dracula"
auto_switch = true

[theme.custom]
accent = "#ff00ff"
"##;
        fs::write(&config_path, initial).unwrap();

        let theme = find_theme("catppuccin").unwrap();
        update_herdr_theme(&config_path, theme).unwrap();

        let updated = fs::read_to_string(&config_path).unwrap();
        assert!(updated.contains(r#"prefix = "ctrl+b""#));
        assert!(updated.contains(r#"name = "catppuccin""#));
        assert!(updated.contains(r#"auto_switch = false"#));
        assert!(updated.contains(r##"accent = "#ff00ff""##));
        assert!(updated.contains(r##"sidebar_bg = "#181825""##));
        assert!(updated.contains("# Initial comment"));

        let _ = fs::remove_dir_all(&tmp_dir);
    }
}
