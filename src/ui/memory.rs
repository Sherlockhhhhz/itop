use ratatui::{
    Frame,
    layout::Rect,
    style::{Modifier, Style},
    symbols,
    text::{Line, Span},
    widgets::{Axis, Block, BorderType, Borders, Chart, Dataset, Gauge, GraphType},
};
use crate::{app::App, theme::*};

pub fn render_mem_gauge(f: &mut Frame, area: Rect, app: &App) {
    let pct = app.mem_pct();
    let color = usage_color(pct);
    let gauge = Gauge::default()
        .block(
            Block::default()
                .title(Line::from(vec![
                    Span::styled(" MEM ", Style::default().fg(COLOR_MAUVE).add_modifier(Modifier::BOLD)),
                    Span::styled(" memory ", Style::default().fg(COLOR_SUBTEXT)),
                ]))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(COLOR_SURFACE))
                .style(Style::default().bg(COLOR_BG)),
        )
        .gauge_style(Style::default().fg(color).bg(COLOR_SURFACE))
        .percent(pct as u16)
        .label(Span::styled(
            format!("{:.1} / {:.1} GB", app.mem_used_gb(), app.mem_total_gb()),
            Style::default().fg(COLOR_TEXT).add_modifier(Modifier::BOLD),
        ));
    f.render_widget(gauge, area);
}

pub fn render_swap_gauge(f: &mut Frame, area: Rect, app: &App) {
    let pct = app.swap_pct();
    let color = usage_color(pct);
    let gauge = Gauge::default()
        .block(
            Block::default()
                .title(Line::from(vec![
                    Span::styled(" SWP ", Style::default().fg(COLOR_SAPPHIRE).add_modifier(Modifier::BOLD)),
                    Span::styled(" swap ", Style::default().fg(COLOR_SUBTEXT)),
                ]))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(COLOR_SURFACE))
                .style(Style::default().bg(COLOR_BG)),
        )
        .gauge_style(Style::default().fg(color).bg(COLOR_SURFACE))
        .percent(pct as u16)
        .label(Span::styled(
            format!("{:.1} / {:.1} GB", app.swap_used_gb(), app.swap_total_gb()),
            Style::default().fg(COLOR_TEXT).add_modifier(Modifier::BOLD),
        ));
    f.render_widget(gauge, area);
}

pub fn render_chart(f: &mut Frame, area: Rect, app: &App) {
    let data: Vec<(f64, f64)> = app.mem_history.iter().cloned().collect();
    let x_min = data.first().map(|d| d.0).unwrap_or(0.0);
    let x_max = data.last().map(|d| d.0).unwrap_or(1.0);

    let datasets = vec![
        Dataset::default()
            .name("mem%")
            .marker(symbols::Marker::Braille)
            .graph_type(GraphType::Line)
            .style(Style::default().fg(COLOR_MAUVE))
            .data(&data),
    ];
    let chart = Chart::new(datasets)
        .block(
            Block::default()
                .title(Line::from(vec![
                    Span::styled(" MEM History ", Style::default().fg(COLOR_MAUVE).add_modifier(Modifier::BOLD)),
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
