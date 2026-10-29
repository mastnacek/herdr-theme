use crate::shared::solar::SolarSlot;
use crate::slices::picker::solar_state::SolarState;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Paragraph},
    Frame,
};

pub fn render_solar(frame: &mut Frame, area: Rect, state: &SolarState) {
    let main_block = Block::bordered()
        .title(" NOAA Astronomical Solar Monitor ")
        .border_style(Style::default().fg(Color::Cyan));

    let inner = main_block.inner(area);
    frame.render_widget(main_block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(7), // Observer & Times
            Constraint::Length(8), // Solar elevation & Phase
            Constraint::Min(4),    // Horizon Gauge & Key
        ])
        .split(inner);

    // 1. Observer & Times
    let obs_lines = vec![
        Line::from(vec![
            Span::styled(" Observer Location:    ", Style::default().fg(Color::White).bold()),
            Span::styled(format!("{:.4}°N, {:.4}°E", state.lat, state.lon), Style::default().fg(Color::Green).bold()),
        ]),
        Line::from(vec![
            Span::styled(" UTC Time:             ", Style::default().fg(Color::White)),
            Span::styled(&state.utc_time, Style::default().fg(Color::Yellow)),
        ]),
        Line::from(vec![
            Span::styled(" Local Time:           ", Style::default().fg(Color::White)),
            Span::styled(&state.local_time, Style::default().fg(Color::Yellow).bold()),
        ]),
    ];
    let obs_block = Block::bordered().title(" Astronomical Ephemeris ");
    frame.render_widget(Paragraph::new(obs_lines).block(obs_block), chunks[0]);

    // 2. Solar Elevation & Phase
    let phase_color = match state.slot {
        SolarSlot::Day => Color::Yellow,
        SolarSlot::Dawn => Color::Cyan,
        SolarSlot::Dusk => Color::Magenta,
        SolarSlot::Night => Color::Blue,
    };

    let elev_color = if state.elevation >= 0.0 {
        Color::Yellow
    } else if state.elevation >= -12.0 {
        Color::Cyan
    } else {
        Color::Blue
    };

    let phase_lines = vec![
        Line::from(vec![
            Span::styled(" Solar Elevation Angle:  ", Style::default().fg(Color::White).bold()),
            Span::styled(format!("{:+6.2}°", state.elevation), Style::default().fg(elev_color).bold()),
            Span::raw(if state.elevation > 0.0 { " (Above Horizon)" } else { " (Below Horizon)" }),
        ]),
        Line::from(vec![
            Span::styled(" Current Solar Slot:     ", Style::default().fg(Color::White).bold()),
            Span::styled(state.slot.as_str().to_uppercase(), Style::default().fg(phase_color).bold()),
        ]),
        Line::from(vec![
            Span::styled(" Active Assigned Theme:  ", Style::default().fg(Color::White).bold()),
            Span::styled(&state.active_theme, Style::default().fg(Color::Green).bold()),
        ]),
    ];
    let phase_block = Block::bordered().title(" Solar Status ");
    frame.render_widget(Paragraph::new(phase_lines).block(phase_block), chunks[1]);

    // 3. ASCII Horizon Gauge
    let gauge_lines = vec![
        Line::from(vec![
            Span::styled(" Astronomical Horizon Scale: ", Style::default().fg(Color::Gray)),
        ]),
        Line::from(vec![
            Span::styled(" [ -90° Night ] ", Style::default().fg(Color::Blue)),
            Span::raw("---"),
            Span::styled(" [ -12° Dawn/Dusk ] ", Style::default().fg(Color::Cyan)),
            Span::raw("---"),
            Span::styled(" [ -0.83° Horizon ] ", Style::default().fg(Color::White)),
            Span::raw("---"),
            Span::styled(" [ +90° Noon Day ] ", Style::default().fg(Color::Yellow)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled(" Press 'r' to force-resync circadian theme now. Press '1' or '2' to switch tabs.", Style::default().fg(Color::DarkGray)),
        ]),
    ];
    frame.render_widget(Paragraph::new(gauge_lines), chunks[2]);
}
