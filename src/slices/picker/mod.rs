pub mod settings_state;
pub mod solar_state;
pub mod state;
pub mod ui;
pub mod ui_settings;
pub mod ui_solar;
pub mod ui_themes;

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use solar_state::SolarState;
use state::{ActiveTab, PickerState};
use std::io::{self, stdout};

pub fn run_popup(initial_tab: ActiveTab) -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut state = PickerState::new(initial_tab);

    loop {
        terminal.draw(|f| {
            ui::render(f, &state);
        })?;

        if let Event::Key(key) = event::read()? {
            // Only handle Press events to avoid duplicate key fires on Windows
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,

                    // Global Tab Switching
                    KeyCode::Char('1') => state.active_tab = ActiveTab::Themes,
                    KeyCode::Char('2') => state.active_tab = ActiveTab::Settings,
                    KeyCode::Char('3') => {
                        state.active_tab = ActiveTab::Solar;
                        state.solar_state = SolarState::refresh();
                    }
                    KeyCode::Tab => {
                        state.active_tab = state.active_tab.next();
                        if state.active_tab == ActiveTab::Solar {
                            state.solar_state = SolarState::refresh();
                        }
                    }
                    KeyCode::BackTab => {
                        state.active_tab = state.active_tab.prev();
                        if state.active_tab == ActiveTab::Solar {
                            state.solar_state = SolarState::refresh();
                        }
                    }

                    // Tab-specific controls
                    _ => match state.active_tab {
                        ActiveTab::Themes => match key.code {
                            KeyCode::Up | KeyCode::Char('k') => {
                                state.previous();
                                state.status_message = None;
                            }
                            KeyCode::Down | KeyCode::Char('j') => {
                                state.next();
                                state.status_message = None;
                            }
                            KeyCode::Char('f') => {
                                state.cycle_filter();
                            }
                            KeyCode::Enter => {
                                let _ = state.apply_current();
                            }
                            KeyCode::Char('c') => {
                                let _ = crate::slices::circadian::run_circadian(true);
                                let active = crate::shared::config::read_current_theme(
                                    &crate::shared::config::default_herdr_config_path(),
                                );
                                state.active_applied_name = active;
                                state.status_message = Some("Circadian theme applied".to_string());
                            }
                            _ => {}
                        },
                        ActiveTab::Settings => match key.code {
                            KeyCode::Up | KeyCode::Char('k') => {
                                state.settings_state.prev_field();
                            }
                            KeyCode::Down | KeyCode::Char('j') => {
                                state.settings_state.next_field();
                            }
                            KeyCode::Left | KeyCode::Char('h') => {
                                state.settings_state.adjust_field(false);
                            }
                            KeyCode::Right | KeyCode::Char('l') => {
                                state.settings_state.adjust_field(true);
                            }
                            KeyCode::Char(' ') | KeyCode::Enter => {
                                state.settings_state.adjust_field(true);
                            }
                            KeyCode::Char('s') => {
                                state.settings_state.save();
                            }
                            KeyCode::Char('r') => {
                                state.settings_state.resync_now();
                            }
                            _ => {}
                        },
                        ActiveTab::Solar => match key.code {
                            KeyCode::Char('r') => {
                                let _ = crate::slices::circadian::run_circadian(true);
                                state.solar_state = SolarState::refresh();
                            }
                            _ => {}
                        },
                    },
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}
