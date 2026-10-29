use crate::shared::config::{default_herdr_config_path, read_current_theme, update_herdr_theme};
use crate::shared::client::HerdrClient;
use crate::shared::terminal::emit_theme_skin;
use crate::shared::theme::{ThemeInfo, ThemeMode, ALL_THEMES};
use std::io;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilterMode {
    All,
    DarkOnly,
    LightOnly,
}

impl FilterMode {
    pub fn next(&self) -> Self {
        match self {
            FilterMode::All => FilterMode::DarkOnly,
            FilterMode::DarkOnly => FilterMode::LightOnly,
            FilterMode::LightOnly => FilterMode::All,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            FilterMode::All => "All Themes",
            FilterMode::DarkOnly => "Dark Only",
            FilterMode::LightOnly => "Light Only",
        }
    }
}

pub struct PickerState {
    pub all_themes: &'static [ThemeInfo],
    pub filtered_themes: Vec<&'static ThemeInfo>,
    pub selected_index: usize,
    pub active_applied_name: Option<String>,
    pub filter_mode: FilterMode,
    pub status_message: Option<String>,
}

impl PickerState {
    pub fn new() -> Self {
        let config_path = default_herdr_config_path();
        let active = read_current_theme(&config_path);

        let mut state = Self {
            all_themes: ALL_THEMES,
            filtered_themes: ALL_THEMES.iter().collect(),
            selected_index: 0,
            active_applied_name: active.clone(),
            filter_mode: FilterMode::All,
            status_message: None,
        };

        // If there is an active theme, pre-select it
        if let Some(active_name) = active {
            if let Some(idx) = state
                .filtered_themes
                .iter()
                .position(|t| t.name == active_name)
            {
                state.selected_index = idx;
            }
        }

        state
    }

    pub fn update_filter(&mut self) {
        self.filtered_themes = self
            .all_themes
            .iter()
            .filter(|t| match self.filter_mode {
                FilterMode::All => true,
                FilterMode::DarkOnly => t.mode == ThemeMode::Dark,
                FilterMode::LightOnly => t.mode == ThemeMode::Light,
            })
            .collect();

        if self.selected_index >= self.filtered_themes.len() {
            self.selected_index = self.filtered_themes.len().saturating_sub(1);
        }
    }

    pub fn cycle_filter(&mut self) {
        self.filter_mode = self.filter_mode.next();
        self.update_filter();
        self.status_message = Some(format!("Filter: {}", self.filter_mode.label()));
    }

    pub fn next(&mut self) {
        if !self.filtered_themes.is_empty() {
            self.selected_index = (self.selected_index + 1) % self.filtered_themes.len();
        }
    }

    pub fn previous(&mut self) {
        if !self.filtered_themes.is_empty() {
            if self.selected_index == 0 {
                self.selected_index = self.filtered_themes.len() - 1;
            } else {
                self.selected_index -= 1;
            }
        }
    }

    pub fn current_selected(&self) -> Option<&'static ThemeInfo> {
        self.filtered_themes.get(self.selected_index).copied()
    }

    pub fn apply_current(&mut self) -> io::Result<()> {
        if let Some(theme) = self.current_selected() {
            let config_path = default_herdr_config_path();
            update_herdr_theme(&config_path, theme)?;

            // Emit terminal skin
            let _ = emit_theme_skin(theme);

            // Reload running Herdr server
            let client = HerdrClient::new();
            let reloaded = client.reload_server();

            self.active_applied_name = Some(theme.name.to_string());
            let msg = if reloaded {
                format!("Applied '{}' & reloaded Herdr", theme.display_name)
            } else {
                format!("Applied '{}' (saved to config)", theme.display_name)
            };
            client.notify("Theme Changed", &msg);
            self.status_message = Some(msg);
        }
        Ok(())
    }
}
