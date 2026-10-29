pub mod state;
pub mod ui;

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use state::PickerState;
use std::io::{self, stdout};

pub fn run_popup() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut state = PickerState::new();

    loop {
        terminal.draw(|f| {
            ui::render(f, &state);
        })?;

        if let Event::Key(key) = event::read()? {
            // Only handle Press events to avoid duplicate key fires on Windows
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    KeyCode::Up | KeyCode::Char('k') => {
                        state.previous();
                        state.status_message = None;
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        state.next();
                        state.status_message = None;
                    }
                    KeyCode::Tab => {
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
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}
