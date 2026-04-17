use ratatui::{
    Frame,
    layout::Rect,
    style::{Modifier, Style},
    symbols,
    text::{Line, Span},
    widgets::{Axis, Block, BorderType, Borders, Chart, Dataset, Gauge, GraphType, Paragraph},
};
use crate::{app::App, theme::*};

pub fn render_gauge(f: &mut Frame, area: Rect, app: &App) {
    let pct = app.cpu_usage();
    let color = usage_color(pct);
    let gauge = Gauge::default()
        .block(
            Block::default()
                .title(Line::from(vec![
                    Span::styled(" CPU ", Style::default().fg(COLOR_BLUE).add_modifier(Modifier::BOLD)),
                    Span::styled(
                        format!(" {} cores ", app.sys.cpus().len()),
                        Style::default().fg(COLOR_SUBTEXT),
                    ),
                ]))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(COLOR_SURFACE))
                .style(Style::default().bg(COLOR_BG)),
        )
        .gauge_style(Style::default().fg(color).bg(COLOR_SURFACE))
        .percent(pct as u16)
        .label(Span::styled(
            format!("{:.1}%", pct),
            Style::default().fg(COLOR_TEXT).add_modifier(Modifier::BOLD),
        ));
    f.render_widget(gauge, area);
}

pub fn render_chart(f: &mut Frame, area: Rect, app: &App) {
    let data: Vec<(f64, f64)> = app.cpu_history.iter().cloned().collect();
    let x_min = data.first().map(|d| d.0).unwrap_or(0.0);
    let x_max = data.last().map(|d| d.0).unwrap_or(1.0);

    let datasets = vec![
        Dataset::default()
            .name("cpu%")
            .marker(symbols::Marker::Braille)
            .graph_type(GraphType::Line)
            .style(Style::default().fg(COLOR_BLUE))
            .data(&data),
    ];
    let chart = Chart::new(datasets)
        .block(
            Block::default()
                .title(Line::from(vec![
                    Span::styled(" CPU History ", Style::default().fg(COLOR_BLUE).add_modifier(Modifier::BOLD)),
                    Span::styled("60s", Style::default().fg(COLOR_SUBTEXT)),
                ]))
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
                    Span::styled("50%", Style::default().fg(COLOR_SUBTEXT)),
                    Span::styled("100%", Style::default().fg(COLOR_SUBTEXT)),
                ])
                .bounds([0.0, 100.0]),
        );
    f.render_widget(chart, area);
}

pub fn render_per_core(f: &mut Frame, area: Rect, app: &App) {
    let cpus = app.per_cpu();
    let col_width = 20usize;
    let avail_w = area.width.saturating_sub(2) as usize;
    let cols = (avail_w / col_width).max(1);

    let lines: Vec<Line> = cpus
        .chunks(cols)
        .enumerate()
        .map(|(row_idx, chunk)| {
            let mut spans: Vec<Span> = Vec::new();
            for (j, &usage) in chunk.iter().enumerate() {
                let idx = row_idx * cols + j;
                let color = usage_color(usage);
                let bar_w = 8usize;
                let filled = (usage / 100.0 * bar_w as f64).round() as usize;
                let bar = "█".repeat(filled) + &"░".repeat(bar_w - filled);
                spans.push(Span::styled(format!("CPU{:<2} ", idx), Style::default().fg(COLOR_SUBTEXT)));
                spans.push(Span::styled(bar, Style::default().fg(color)));
                spans.push(Span::styled(format!(" {:5.1}%  ", usage), Style::default().fg(COLOR_TEXT)));
            }
            Line::from(spans)
        })
        .collect();

    let p = Paragraph::new(lines).block(
        Block::default()
            .title(Line::from(vec![Span::styled(
                " Per-Core ",
                Style::default().fg(COLOR_SAPPHIRE).add_modifier(Modifier::BOLD),
            )]))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(COLOR_SURFACE))
            .style(Style::default().bg(COLOR_BG)),
    );
    f.render_widget(p, area);
}
