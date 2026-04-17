use ratatui::{
    Frame,
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
};
use crate::{app::App, theme::*};

pub fn render(f: &mut Frame, area: Rect, _app: &App) {
    let hostname = sysinfo::System::host_name().unwrap_or_else(|| "unknown".into());
    let os = sysinfo::System::long_os_version().unwrap_or_else(|| "unknown".into());
    let kernel = sysinfo::System::kernel_version().unwrap_or_else(|| "unknown".into());
    let uptime_s = sysinfo::System::uptime();
    let uptime = format!("{}h {}m {}s", uptime_s / 3600, (uptime_s % 3600) / 60, uptime_s % 60);

    let lines = vec![
        Line::from(vec![
            Span::styled("  host   ", Style::default().fg(COLOR_OVERLAY)),
            Span::styled(hostname, Style::default().fg(COLOR_LAVENDER).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled("  os     ", Style::default().fg(COLOR_OVERLAY)),
            Span::styled(os, Style::default().fg(COLOR_TEXT)),
        ]),
        Line::from(vec![
            Span::styled("  kernel ", Style::default().fg(COLOR_OVERLAY)),
            Span::styled(kernel, Style::default().fg(COLOR_TEXT)),
        ]),
        Line::from(vec![
            Span::styled("  uptime ", Style::default().fg(COLOR_OVERLAY)),
            Span::styled(uptime, Style::default().fg(COLOR_GREEN)),
        ]),
    ];

    let p = Paragraph::new(lines).block(
        Block::default()
            .title(Line::from(vec![Span::styled(
                " System ",
                Style::default().fg(COLOR_LAVENDER).add_modifier(Modifier::BOLD),
            )]))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(COLOR_SURFACE))
            .style(Style::default().bg(COLOR_BG)),
    );
    f.render_widget(p, area);
}
