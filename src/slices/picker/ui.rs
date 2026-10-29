use crate::slices::picker::state::PickerState;
use crate::shared::theme::ThemeMode;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Clear, List, ListItem, Paragraph},
    Frame,
};
use std::str::FromStr;

pub fn render(frame: &mut Frame, state: &PickerState) {
    let area = frame.area();

    // Clear outer area so popup is clean
    frame.render_widget(Clear, area);

    let main_block = Block::bordered()
        .title(" Herdr Theme Switcher ")
        .title_style(Style::default().fg(Color::Cyan).bold())
        .border_style(Style::default().fg(Color::DarkGray));

    let inner = main_block.inner(area);
    frame.render_widget(main_block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Sub-header / status
            Constraint::Min(8),    // Main list & details
            Constraint::Length(2), // Footer
        ])
        .split(inner);

    // 1. Header Row
    let active_str = state
        .active_applied_name
        .as_deref()
        .unwrap_or("None (Default)");

    let header_line = Line::from(vec![
        Span::raw(" Active: "),
        Span::styled(active_str, Style::default().fg(Color::Green).bold()),
        Span::raw("   |   Filter: "),
        Span::styled(
            state.filter_mode.label(),
            Style::default().fg(Color::Yellow).bold(),
        ),
        Span::raw(" (Press Tab to cycle)"),
    ]);
    frame.render_widget(Paragraph::new(header_line), chunks[0]);

    // 2. Middle Body: Left (List) & Right (Preview Details)
    let body_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(46), Constraint::Percentage(54)])
        .split(chunks[1]);

    // Left: Theme List
    let items: Vec<ListItem> = state
        .filtered_themes
        .iter()
        .enumerate()
        .map(|(i, t)| {
            let is_selected = i == state.selected_index;
            let is_active = state.active_applied_name.as_deref() == Some(t.name);

            let mode_badge = match t.mode {
                ThemeMode::Dark => Span::styled(" [D] ", Style::default().fg(Color::Blue)),
                ThemeMode::Light => Span::styled(" [L] ", Style::default().fg(Color::Yellow)),
            };

            let prefix = if is_selected { "▶ " } else { "  " };

            let name_style = if is_selected {
                Style::default().fg(Color::White).bold()
            } else {
                Style::default().fg(Color::Gray)
            };

            let mut spans = vec![
                Span::styled(prefix, Style::default().fg(Color::Cyan).bold()),
                mode_badge,
                Span::styled(t.display_name, name_style),
            ];

            if is_active {
                spans.push(Span::styled(" ★", Style::default().fg(Color::Green).bold()));
            }

            if let Some(slot) = t.circadian_slot {
                spans.push(Span::styled(
                    format!(" ({})", slot),
                    Style::default().fg(Color::Magenta).italic(),
                ));
            }

            let item_style = if is_selected {
                Style::default().bg(Color::Rgb(40, 44, 52))
            } else {
                Style::default()
            };

            ListItem::new(Line::from(spans)).style(item_style)
        })
        .collect();

    let list_block = Block::bordered()
        .title(" Available Themes ")
        .border_style(Style::default().fg(Color::DarkGray));
    let list_widget = List::new(items).block(list_block);
    frame.render_widget(list_widget, body_chunks[0]);

    // Right: Preview & Details
    let preview_block = Block::bordered()
        .title(" Palette & Preview ")
        .border_style(Style::default().fg(Color::DarkGray));

    let preview_inner = preview_block.inner(body_chunks[1]);
    frame.render_widget(preview_block, body_chunks[1]);

    if let Some(theme) = state.current_selected() {
        let accent_color = Color::from_str(theme.accent_hex).unwrap_or(Color::Cyan);
        let bg_color = Color::from_str(theme.bg_hex).unwrap_or(Color::Black);
        let fg_color = Color::from_str(theme.fg_hex).unwrap_or(Color::White);
        let sidebar_color = Color::from_str(theme.sidebar_bg).unwrap_or(Color::DarkGray);

        let details = vec![
            Line::from(vec![
                Span::styled(theme.display_name, Style::default().fg(accent_color).bold()),
                Span::raw(format!("  ({})", theme.name)),
            ]),
            Line::from(vec![
                Span::raw("Mode: "),
                Span::styled(
                    match theme.mode {
                        ThemeMode::Dark => "Dark Palette",
                        ThemeMode::Light => "Light Palette",
                    },
                    Style::default().bold(),
                ),
                Span::raw(if let Some(s) = theme.circadian_slot {
                    format!("  |  Circadian Slot: {}", s.to_uppercase())
                } else {
                    String::new()
                }),
            ]),
            Line::raw(""),
            Line::from(vec![
                Span::raw("Background:  "),
                Span::styled("   ", Style::default().bg(bg_color)),
                Span::raw(format!(" {}", theme.bg_hex)),
            ]),
            Line::from(vec![
                Span::raw("Foreground:  "),
                Span::styled("   ", Style::default().bg(fg_color)),
                Span::raw(format!(" {}", theme.fg_hex)),
            ]),
            Line::from(vec![
                Span::raw("Sidebar BG:  "),
                Span::styled("   ", Style::default().bg(sidebar_color)),
                Span::raw(format!(" {}", theme.sidebar_bg)),
            ]),
            Line::from(vec![
                Span::raw("Accent Color:"),
                Span::styled("   ", Style::default().bg(accent_color)),
                Span::raw(format!(" {}", theme.accent_hex)),
            ]),
            Line::raw(""),
            // Simulated Mini Window
            Line::from(vec![
                Span::styled(" [Sidebar] ", Style::default().bg(sidebar_color).fg(fg_color)),
                Span::styled(" [Terminal Pane Content] ", Style::default().bg(bg_color).fg(fg_color)),
            ]),
        ];

        frame.render_widget(Paragraph::new(details), preview_inner);
    }

    // 3. Footer Row
    let footer_line = if let Some(ref msg) = state.status_message {
        Line::from(vec![
            Span::styled("✓ ", Style::default().fg(Color::Green).bold()),
            Span::styled(msg, Style::default().fg(Color::Green).bold()),
        ])
    } else {
        Line::from(vec![
            Span::styled("[↑/↓/j/k]", Style::default().fg(Color::Yellow).bold()),
            Span::raw(" Move  "),
            Span::styled("[Enter]", Style::default().fg(Color::Green).bold()),
            Span::raw(" Apply  "),
            Span::styled("[Tab]", Style::default().fg(Color::Cyan).bold()),
            Span::raw(" Filter  "),
            Span::styled("[c]", Style::default().fg(Color::Magenta).bold()),
            Span::raw(" Circadian  "),
            Span::styled("[q/Esc]", Style::default().fg(Color::Red).bold()),
            Span::raw(" Close"),
        ])
    };

    frame.render_widget(Paragraph::new(footer_line), chunks[2]);
}
