use ratatui::{
    Frame,
    layout::Rect,
    style::{Modifier, Style},
    symbols,
    text::{Line, Span},
    widgets::{Axis, Block, BorderType, Borders, Chart, Dataset, Gauge, GraphType, Paragraph},
};
use crate::{app::App, theme::*};

pub fn render(f: &mut Frame, area: Rect, app: &App) {
    match &app.gpu {
        Some(gpu) => render_live(f, area, app, gpu.utilization_pct, &gpu.device_name),
        None => render_unavailable(f, area),
    }
}

fn render_live(f: &mut Frame, area: Rect, app: &App, util_pct: f64, device: &str) {
    use ratatui::layout::{Constraint, Direction, Layout};

    let inner = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(4)])
        .split(area);

    // Gauge in the top part
    let color = usage_color(util_pct);
    let gauge = Gauge::default()
        .block(
            Block::default()
                .title(Line::from(vec![
                    Span::styled(" GPU ", Style::default().fg(COLOR_PEACH).add_modifier(Modifier::BOLD)),
                    Span::styled(format!(" {} ", device), Style::default().fg(COLOR_SUBTEXT)),
                ]))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(COLOR_SURFACE))
                .style(Style::default().bg(COLOR_BG)),
        )
        .gauge_style(Style::default().fg(color).bg(COLOR_SURFACE))
        .percent(util_pct as u16)
        .label(Span::styled(
            format!("{:.1}%", util_pct),
            Style::default().fg(COLOR_TEXT).add_modifier(Modifier::BOLD),
        ));
    f.render_widget(gauge, inner[0]);

    // History chart in the bottom part
    let data: Vec<(f64, f64)> = app.gpu_history.iter().cloned().collect();
    let x_min = data.first().map(|d| d.0).unwrap_or(0.0);
    let x_max = data.last().map(|d| d.0).unwrap_or(1.0);
    let datasets = vec![
        Dataset::default()
            .name("gpu%")
            .marker(symbols::Marker::Braille)
            .graph_type(GraphType::Line)
            .style(Style::default().fg(COLOR_PEACH))
            .data(&data),
    ];
    let chart = Chart::new(datasets)
        .block(
            Block::default()
                .title(Span::styled(" GPU History ", Style::default().fg(COLOR_PEACH).add_modifier(Modifier::BOLD)))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(COLOR_SURFACE))
                .style(Style::default().bg(COLOR_BG)),
        )
        .x_axis(Axis::default().style(Style::default().fg(COLOR_OVERLAY)).bounds([x_min, x_max]))
        .y_axis(
            Axis::default()
                .style(Style::default().fg(COLOR_OVERLAY))
                .labels(vec![
                    Span::styled("0%", Style::default().fg(COLOR_SUBTEXT)),
                    Span::styled("100%", Style::default().fg(COLOR_SUBTEXT)),
                ])
                .bounds([0.0, 100.0]),
        );
    f.render_widget(chart, inner[1]);
}

fn render_unavailable(f: &mut Frame, area: Rect) {
    let lines = vec![
        Line::from(vec![Span::styled(
            "  Apple Silicon MPS",
            Style::default().fg(COLOR_SUBTEXT),
        )]),
        Line::from(vec![
            Span::styled("  querying via ", Style::default().fg(COLOR_SUBTEXT)),
            Span::styled("ioreg", Style::default().fg(COLOR_PEACH)),
            Span::styled("...", Style::default().fg(COLOR_SUBTEXT)),
        ]),
        Line::from(vec![Span::styled(
            "  (no IOAccelerator found)",
            Style::default().fg(COLOR_OVERLAY).add_modifier(Modifier::ITALIC),
        )]),
    ];
    let p = Paragraph::new(lines).block(
        Block::default()
            .title(Line::from(vec![
                Span::styled(" GPU ", Style::default().fg(COLOR_PEACH).add_modifier(Modifier::BOLD)),
                Span::styled(" (Metal/MPS) ", Style::default().fg(COLOR_SUBTEXT)),
            ]))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(COLOR_SURFACE))
            .style(Style::default().bg(COLOR_BG)),
    );
    f.render_widget(p, area);
}
