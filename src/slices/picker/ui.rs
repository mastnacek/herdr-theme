use crate::slices::picker::state::{ActiveTab, PickerState};
use crate::slices::picker::ui_settings::render_settings;
use crate::slices::picker::ui_solar::render_solar;
use crate::slices::picker::ui_themes::render_themes;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Clear, Paragraph},
    Frame,
};

pub fn render(frame: &mut Frame, state: &PickerState) {
    let area = frame.area();

    // Clear outer area so popup is clean
    frame.render_widget(Clear, area);

    let main_block = Block::bordered()
        .title(" Herdr Theme & Environment Manager ")
        .title_style(Style::default().fg(Color::Cyan).bold())
        .border_style(Style::default().fg(Color::DarkGray));

    let inner = main_block.inner(area);
    frame.render_widget(main_block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Tab Bar
            Constraint::Min(8),    // Active Tab Content
            Constraint::Length(1), // Footer Shortcuts
        ])
        .split(inner);

    // 1. Tab Bar
    let render_tab = |tab: ActiveTab, label: &str| -> Span<'static> {
        if state.active_tab == tab {
            Span::styled(format!(" [ {} ] ", label), Style::default().fg(Color::Black).bg(Color::Cyan).bold())
        } else {
            Span::styled(format!("   {}   ", label), Style::default().fg(Color::Gray))
        }
    };

    let tab_line = Line::from(vec![
        Span::raw(" Tabs: "),
        render_tab(ActiveTab::Themes, "1: Themes"),
        Span::raw(" "),
        render_tab(ActiveTab::Settings, "2: Settings"),
        Span::raw(" "),
        render_tab(ActiveTab::Solar, "3: Solar Clock"),
        Span::raw("    (Press 1/2/3 or Tab)"),
    ]);
    frame.render_widget(Paragraph::new(tab_line), chunks[0]);

    // 2. Active Tab Content
    match state.active_tab {
        ActiveTab::Themes => render_themes(frame, chunks[1], state),
        ActiveTab::Settings => render_settings(frame, chunks[1], &state.settings_state),
        ActiveTab::Solar => render_solar(frame, chunks[1], &state.solar_state),
    }

    // 3. Footer Shortcuts
    let footer_spans = match state.active_tab {
        ActiveTab::Themes => vec![
            Span::styled(" [↑/↓/j/k] ", Style::default().fg(Color::Yellow)),
            Span::raw("Select  "),
            Span::styled(" [Enter] ", Style::default().fg(Color::Green)),
            Span::raw("Apply  "),
            Span::styled(" [f] ", Style::default().fg(Color::Cyan)),
            Span::raw("Filter  "),
            Span::styled(" [c] ", Style::default().fg(Color::Magenta)),
            Span::raw("Sync Circadian  "),
            Span::styled(" [Tab] ", Style::default().fg(Color::White)),
            Span::raw("Next Tab  "),
            Span::styled(" [q/Esc] ", Style::default().fg(Color::Red)),
            Span::raw("Quit"),
        ],
        ActiveTab::Settings => vec![
            Span::styled(" [↑/↓/j/k] ", Style::default().fg(Color::Yellow)),
            Span::raw("Navigate  "),
            Span::styled(" [←/→/h/l] ", Style::default().fg(Color::Cyan)),
            Span::raw("Change Value  "),
            Span::styled(" [Space/Enter] ", Style::default().fg(Color::Green)),
            Span::raw("Toggle  "),
            Span::styled(" [s] ", Style::default().fg(Color::Green).bold()),
            Span::raw("Save  "),
            Span::styled(" [r] ", Style::default().fg(Color::Magenta)),
            Span::raw("Resync Now  "),
            Span::styled(" [q/Esc] ", Style::default().fg(Color::Red)),
            Span::raw("Quit"),
        ],
        ActiveTab::Solar => vec![
            Span::styled(" [r] ", Style::default().fg(Color::Magenta)),
            Span::raw("Resync Circadian Now  "),
            Span::styled(" [Tab] ", Style::default().fg(Color::White)),
            Span::raw("Next Tab  "),
            Span::styled(" [q/Esc] ", Style::default().fg(Color::Red)),
            Span::raw("Quit"),
        ],
    };
    frame.render_widget(Paragraph::new(Line::from(footer_spans)), chunks[2]);
}
