pub mod header;
pub mod cpu;
pub mod memory;
pub mod gpu_panel;
pub mod sysinfo_panel;

use ratatui::{Frame, layout::{Constraint, Direction, Layout}, style::Style, widgets::Block};
use crate::{app::App, theme::COLOR_BG};

pub fn draw(f: &mut Frame, app: &App) {
    let full = f.area();
    f.render_widget(Block::default().style(Style::default().bg(COLOR_BG)), full);

    let root = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2), // header
            Constraint::Length(3), // cpu gauge
            Constraint::Length(3), // mem gauge
            Constraint::Length(3), // swap gauge
            Constraint::Min(8),    // charts row
            Constraint::Min(6),    // bottom row
        ])
        .split(full);

    header::render(f, root[0]);
    cpu::render_gauge(f, root[1], app);
    memory::render_mem_gauge(f, root[2], app);
    memory::render_swap_gauge(f, root[3], app);

    let charts_row = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(root[4]);
    cpu::render_chart(f, charts_row[0], app);
    memory::render_chart(f, charts_row[1], app);

    let bottom_row = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(40),
            Constraint::Percentage(30),
            Constraint::Percentage(30),
        ])
        .split(root[5]);
    cpu::render_per_core(f, bottom_row[0], app);
    sysinfo_panel::render(f, bottom_row[1], app);
    gpu_panel::render(f, bottom_row[2], app);
}
