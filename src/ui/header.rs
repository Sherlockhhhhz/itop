use ratatui::{
    Frame,
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
};
use crate::theme::*;

pub fn render(f: &mut Frame, area: Rect) {
    let title = vec![
        Span::styled(" ⚡ ", Style::default().fg(COLOR_YELLOW)),
        Span::styled("itop", Style::default().fg(COLOR_LAVENDER).add_modifier(Modifier::BOLD)),
        Span::styled("  system monitor", Style::default().fg(COLOR_SUBTEXT)),
        Span::styled("  [q] quit  [r] refresh", Style::default().fg(COLOR_OVERLAY)),
    ];
    let p = Paragraph::new(Line::from(title)).block(
        Block::default()
            .borders(Borders::BOTTOM)
            .border_style(Style::default().fg(COLOR_SURFACE))
            .border_type(BorderType::Plain),
    );
    f.render_widget(p, area);
}
