use crate::shared::theme::ThemeMode;
use crate::slices::picker::state::PickerState;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, List, ListItem, Paragraph},
    Frame,
};
use std::str::FromStr;

pub fn render_themes(frame: &mut Frame, area: Rect, state: &PickerState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Min(6)])
        .split(area);

    // Filter indicator
    let filter_line = Line::from(vec![
        Span::raw(" Filter: "),
        Span::styled(state.filter_mode.label(), Style::default().fg(Color::Yellow).bold()),
        Span::raw(" (Press 'f' to cycle All / Dark / Light)"),
    ]);
    frame.render_widget(Paragraph::new(filter_line), chunks[0]);

    // Body: Left (List) & Right (Preview Details)
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

            let prefix = if is_selected { " ▶ " } else { "   " };
            let active_mark = if is_active { " ●" } else { "  " };

            let mut style = Style::default();
            if is_selected {
                style = style.fg(Color::Black).bg(Color::Cyan).bold();
            } else if is_active {
                style = style.fg(Color::Green).bold();
            }

            ListItem::new(Line::from(vec![
                Span::styled(prefix, style),
                mode_badge,
                Span::styled(format!("{:<18}", t.display_name), style),
                Span::styled(active_mark, if is_active { Style::default().fg(Color::Green) } else { Style::default() }),
            ]))
        })
        .collect();

    let list_title = format!(" Available Themes ({}) ", state.filtered_themes.len());
    let list_block = Block::bordered()
        .title(list_title)
        .border_style(Style::default().fg(Color::Gray));

    let list = List::new(items).block(list_block);
    frame.render_widget(list, body_chunks[0]);

    // Right: Selected Theme Preview
    if let Some(t) = state.selected_theme() {
        let bg_color = Color::from_str(t.bg_hex).unwrap_or(Color::Black);
        let fg_color = Color::from_str(t.fg_hex).unwrap_or(Color::White);
        let sidebar_color = Color::from_str(t.sidebar_bg).unwrap_or(Color::DarkGray);

        let preview_block = Block::bordered()
            .title(format!(" Preview: {} ", t.display_name))
            .border_style(Style::default().fg(Color::Cyan));

        let mut lines = vec![
            Line::from(vec![
                Span::raw(" Identifier:   "),
                Span::styled(t.name, Style::default().fg(Color::White).bold()),
            ]),
            Line::from(vec![
                Span::raw(" Mode:         "),
                match t.mode {
                    ThemeMode::Dark => Span::styled("Dark", Style::default().fg(Color::Blue).bold()),
                    ThemeMode::Light => Span::styled("Light", Style::default().fg(Color::Yellow).bold()),
                },
            ]),
            Line::from(vec![
                Span::raw(" Circadian:    "),
                Span::styled(
                    t.circadian_slot.unwrap_or("none"),
                    Style::default().fg(Color::Magenta),
                ),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::raw(" Background:   "),
                Span::styled(format!("  {}  ", t.bg_hex), Style::default().bg(bg_color).fg(fg_color).bold()),
                Span::raw(format!("  Hex: {}", t.bg_hex)),
            ]),
            Line::from(vec![
                Span::raw(" Foreground:   "),
                Span::styled(" AaBbCc ", Style::default().bg(bg_color).fg(fg_color).bold()),
                Span::raw(format!("  Hex: {}", t.fg_hex)),
            ]),
            Line::from(vec![
                Span::raw(" Sidebar BG:   "),
                Span::styled("        ", Style::default().bg(sidebar_color)),
                Span::raw(format!("  Hex: {}", t.sidebar_bg)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled(" [HERDR TERMINAL CANVAS PREVIEW] ", Style::default().bg(bg_color).fg(fg_color).bold()),
            ]),
            Line::from(vec![
                Span::styled(" 12:34:56 [pi:agent] cargo check -> OK ", Style::default().bg(bg_color).fg(fg_color)),
            ]),
            Line::from(vec![
                Span::styled(" active branch: master | 0 errors      ", Style::default().bg(bg_color).fg(fg_color)),
            ]),
        ];

        if let Some(msg) = &state.status_message {
            lines.push(Line::from(""));
            lines.push(Line::from(vec![
                Span::styled(format!(" ★ {}", msg), Style::default().fg(Color::Green).bold()),
            ]));
        }

        let preview = Paragraph::new(lines).block(preview_block);
        frame.render_widget(preview, body_chunks[1]);
    }
}
