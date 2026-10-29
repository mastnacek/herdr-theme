use crate::shared::theme::{find_theme, ThemeMode};
use crate::slices::picker::settings_state::{
    SettingsField, SettingsState, PRESET_CITIES,
};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Paragraph},
    Frame,
};
use std::str::FromStr;

pub fn render_settings(frame: &mut Frame, area: Rect, state: &SettingsState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // dirty / status banner
            Constraint::Min(10),   // Form fields
        ])
        .split(area);

    // 1. Status / Dirty banner
    let mut banner_spans = Vec::new();
    if state.is_dirty {
        banner_spans.push(Span::styled(" ● Unsaved Changes (Press 's' to Save) ", Style::default().fg(Color::Yellow).bold()));
    } else {
        banner_spans.push(Span::styled(" ✔ Configuration in sync with settings.toml ", Style::default().fg(Color::Green)));
    }

    if let Some((msg, is_err)) = &state.status_message {
        banner_spans.push(Span::raw(" | "));
        if *is_err {
            banner_spans.push(Span::styled(msg, Style::default().fg(Color::Red).bold()));
        } else {
            banner_spans.push(Span::styled(msg, Style::default().fg(Color::Cyan).bold()));
        }
    }
    frame.render_widget(Paragraph::new(Line::from(banner_spans)), chunks[0]);

    // 2. Form Layout: Split into Left Column (Circadian & Location) & Right Column (Daemon & Terminal & Actions)
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[1]);

    // Helper for rendering field line
    let render_field_row = |field: SettingsField, label: &str, value_spans: Vec<Span<'static>>, preview_theme: Option<&str>| -> Vec<Line<'static>> {
        let is_focused = state.focused_field == field as usize;
        let prefix = if is_focused { " ▶ " } else { "   " };
        let label_style = if is_focused {
            Style::default().fg(Color::Cyan).bold()
        } else {
            Style::default().fg(Color::White)
        };

        let mut spans = vec![
            Span::styled(prefix, if is_focused { Style::default().fg(Color::Cyan).bold() } else { Style::default() }),
            Span::styled(format!("{:<20}", label), label_style),
        ];
        spans.extend(value_spans);

        let mut lines = vec![Line::from(spans)];

        // If field has an associated theme preview, render small badge below
        if let Some(tname) = preview_theme {
            if let Some(t) = find_theme(tname) {
                let bg_c = Color::from_str(t.bg_hex).unwrap_or(Color::Black);
                let fg_c = Color::from_str(t.fg_hex).unwrap_or(Color::White);
                lines.push(Line::from(vec![
                    Span::raw("                       Swatch: "),
                    Span::styled(" AaBbCc ", Style::default().bg(bg_c).fg(fg_c).bold()),
                    Span::raw(format!("  {} ({})", t.display_name, if t.mode == ThemeMode::Dark { "Dark" } else { "Light" })),
                ]));
            }
        }

        lines
    };

    // --- LEFT COLUMN ---
    let left_block = Block::bordered()
        .title(" Circadian & Location ")
        .border_style(Style::default().fg(Color::Gray));

    let mut left_lines = Vec::new();

    // 1. Dawn Theme
    left_lines.extend(render_field_row(
        SettingsField::DawnTheme,
        "Dawn Theme:",
        vec![Span::styled(format!("< {:<16} >", state.settings.circadian.dawn), Style::default().fg(Color::Yellow).bold())],
        Some(&state.settings.circadian.dawn),
    ));
    left_lines.push(Line::from(""));

    // 2. Day Theme
    left_lines.extend(render_field_row(
        SettingsField::DayTheme,
        "Day Theme:",
        vec![Span::styled(format!("< {:<16} >", state.settings.circadian.day), Style::default().fg(Color::Yellow).bold())],
        Some(&state.settings.circadian.day),
    ));
    left_lines.push(Line::from(""));

    // 3. Dusk Theme
    left_lines.extend(render_field_row(
        SettingsField::DuskTheme,
        "Dusk Theme:",
        vec![Span::styled(format!("< {:<16} >", state.settings.circadian.dusk), Style::default().fg(Color::Yellow).bold())],
        Some(&state.settings.circadian.dusk),
    ));
    left_lines.push(Line::from(""));

    // 4. Night Theme
    left_lines.extend(render_field_row(
        SettingsField::NightTheme,
        "Night Theme:",
        vec![Span::styled(format!("< {:<16} >", state.settings.circadian.night), Style::default().fg(Color::Yellow).bold())],
        Some(&state.settings.circadian.night),
    ));
    left_lines.push(Line::from(""));

    // 5. City Preset
    let city_name = PRESET_CITIES.get(state.selected_city_idx).map(|c| c.name).unwrap_or("Custom");
    left_lines.extend(render_field_row(
        SettingsField::CityPreset,
        "City Preset:",
        vec![Span::styled(format!("< {:<16} >", city_name), Style::default().fg(Color::Green).bold())],
        None,
    ));

    // 6. Latitude
    left_lines.extend(render_field_row(
        SettingsField::Latitude,
        "Latitude:",
        vec![Span::styled(format!("{:.4}°N  (adjust -/+)", state.settings.location.latitude), Style::default().fg(Color::White).bold())],
        None,
    ));

    // 7. Longitude
    left_lines.extend(render_field_row(
        SettingsField::Longitude,
        "Longitude:",
        vec![Span::styled(format!("{:.4}°E  (adjust -/+)", state.settings.location.longitude), Style::default().fg(Color::White).bold())],
        None,
    ));

    let left_para = Paragraph::new(left_lines).block(left_block);
    frame.render_widget(left_para, cols[0]);

    // --- RIGHT COLUMN ---
    let right_block = Block::bordered()
        .title(" Daemon & Terminal Behavior ")
        .border_style(Style::default().fg(Color::Gray));

    let mut right_lines = Vec::new();

    // 8. Watch Interval
    right_lines.extend(render_field_row(
        SettingsField::WatchInterval,
        "Watch Interval:",
        vec![Span::styled(format!("< {}s >", state.settings.daemon.interval_seconds), Style::default().fg(Color::Yellow).bold())],
        None,
    ));
    right_lines.push(Line::from(""));

    // 9. Skin Terminal
    let skin_badge = if state.settings.terminal.skin_terminal {
        Span::styled("[●] Enabled  (OSC 10/11)", Style::default().fg(Color::Green).bold())
    } else {
        Span::styled("[○] Disabled", Style::default().fg(Color::DarkGray))
    };
    right_lines.extend(render_field_row(
        SettingsField::SkinTerminal,
        "Skin Terminal:",
        vec![skin_badge],
        None,
    ));
    right_lines.push(Line::from(""));

    // 10. Desktop Notify
    let notify_badge = if state.settings.terminal.notify {
        Span::styled("[●] Enabled  (Herdr toast)", Style::default().fg(Color::Green).bold())
    } else {
        Span::styled("[○] Disabled", Style::default().fg(Color::DarkGray))
    };
    right_lines.extend(render_field_row(
        SettingsField::DesktopNotify,
        "Notifications:",
        vec![notify_badge],
        None,
    ));
    right_lines.push(Line::from(""));

    // 11. Daemon Status & Toggle
    let daemon_status_badge = if state.daemon_running {
        vec![
            Span::styled("● RUNNING", Style::default().fg(Color::Green).bold()),
            Span::raw(format!(" (PID: {}) ", state.daemon_pid.unwrap_or(0))),
            Span::styled("[Press Enter to Stop]", Style::default().fg(Color::Red)),
        ]
    } else {
        vec![
            Span::styled("○ STOPPED", Style::default().fg(Color::DarkGray).bold()),
            Span::raw(" "),
            Span::styled("[Press Enter to Start]", Style::default().fg(Color::Green)),
        ]
    };
    right_lines.extend(render_field_row(
        SettingsField::DaemonControl,
        "Daemon Process:",
        daemon_status_badge,
        None,
    ));
    right_lines.push(Line::from(""));
    right_lines.push(Line::from("---------------------------------------------"));
    right_lines.push(Line::from(""));

    // 12. Save Button
    let is_save_focused = state.focused_field == SettingsField::SaveButton as usize;
    let save_style = if is_save_focused {
        Style::default().fg(Color::Black).bg(Color::Green).bold()
    } else {
        Style::default().fg(Color::Green).bold()
    };
    right_lines.push(Line::from(vec![
        Span::raw(if is_save_focused { " ▶ " } else { "   " }),
        Span::styled(" [ SAVE SETTINGS TO DISK ] ", save_style),
        Span::raw(" (Hotkeys: Enter or 's')"),
    ]));
    right_lines.push(Line::from(""));

    // 13. Resync Button
    let is_resync_focused = state.focused_field == SettingsField::ResyncButton as usize;
    let resync_style = if is_resync_focused {
        Style::default().fg(Color::Black).bg(Color::Cyan).bold()
    } else {
        Style::default().fg(Color::Cyan).bold()
    };
    right_lines.push(Line::from(vec![
        Span::raw(if is_resync_focused { " ▶ " } else { "   " }),
        Span::styled(" [ RESYNC CIRCADIAN THEME NOW ] ", resync_style),
        Span::raw(" (Hotkeys: Enter or 'r')"),
    ]));

    let right_para = Paragraph::new(right_lines).block(right_block);
    frame.render_widget(right_para, cols[1]);
}
