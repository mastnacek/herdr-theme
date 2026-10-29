use crate::shared::client::HerdrClient;
use crate::shared::config::{default_herdr_config_path, read_current_theme, update_herdr_theme};
use crate::shared::terminal::emit_theme_skin;
use crate::shared::theme::{ThemeInfo, ThemeMode, ALL_THEMES};
use crate::slices::picker::settings_state::SettingsState;
use crate::slices::picker::solar_state::SolarState;
use std::io;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActiveTab {
    Themes = 0,
    Settings = 1,
    Solar = 2,
}

impl ActiveTab {
    pub fn next(&self) -> Self {
        match self {
            ActiveTab::Themes => ActiveTab::Settings,
            ActiveTab::Settings => ActiveTab::Solar,
            ActiveTab::Solar => ActiveTab::Themes,
        }
    }

    pub fn prev(&self) -> Self {
        match self {
            ActiveTab::Themes => ActiveTab::Solar,
            ActiveTab::Settings => ActiveTab::Themes,
            ActiveTab::Solar => ActiveTab::Settings,
        }
    }
}

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
    pub active_tab: ActiveTab,
    pub all_themes: &'static [ThemeInfo],
    pub filtered_themes: Vec<&'static ThemeInfo>,
    pub selected_index: usize,
    pub active_applied_name: Option<String>,
    pub filter_mode: FilterMode,
    pub status_message: Option<String>,
    pub settings_state: SettingsState,
    pub solar_state: SolarState,
}

impl PickerState {
    pub fn new(initial_tab: ActiveTab) -> Self {
        let config_path = default_herdr_config_path();
        let active = read_current_theme(&config_path);

        let mut state = Self {
            active_tab: initial_tab,
            all_themes: ALL_THEMES,
            filtered_themes: ALL_THEMES.iter().collect(),
            selected_index: 0,
            active_applied_name: active.clone(),
            filter_mode: FilterMode::All,
            status_message: None,
            settings_state: SettingsState::new(),
            solar_state: SolarState::refresh(),
        };

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

        if self.selected_index >= self.filtered_themes.len() && !self.filtered_themes.is_empty() {
            self.selected_index = self.filtered_themes.len() - 1;
        }
    }

    pub fn cycle_filter(&mut self) {
        self.filter_mode = self.filter_mode.next();
        self.update_filter();
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

    pub fn selected_theme(&self) -> Option<&'static ThemeInfo> {
        self.filtered_themes.get(self.selected_index).copied()
    }

    pub fn apply_current(&mut self) -> io::Result<bool> {
        if let Some(theme) = self.selected_theme() {
            let config_path = default_herdr_config_path();
            update_herdr_theme(&config_path, theme)?;

            // Emit OSC 10/11 skinning
            let _ = emit_theme_skin(theme);

            // Reload running Herdr server
            let client = HerdrClient::new();
            client.reload_server();

            self.active_applied_name = Some(theme.name.to_string());
            self.status_message = Some(format!("Applied '{}'", theme.display_name));
            return Ok(true);
        }
        Ok(false)
    }
}
