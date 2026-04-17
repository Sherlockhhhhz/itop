use ratatui::style::Color;

pub const COLOR_BG: Color = Color::Rgb(30, 30, 46);
pub const COLOR_SURFACE: Color = Color::Rgb(49, 50, 68);
pub const COLOR_OVERLAY: Color = Color::Rgb(88, 91, 112);
pub const COLOR_TEXT: Color = Color::Rgb(205, 214, 244);
pub const COLOR_SUBTEXT: Color = Color::Rgb(166, 173, 200);
pub const COLOR_LAVENDER: Color = Color::Rgb(180, 190, 254);
pub const COLOR_BLUE: Color = Color::Rgb(137, 180, 250);
pub const COLOR_SAPPHIRE: Color = Color::Rgb(116, 199, 236);
pub const COLOR_GREEN: Color = Color::Rgb(166, 227, 161);
pub const COLOR_YELLOW: Color = Color::Rgb(249, 226, 175);
pub const COLOR_PEACH: Color = Color::Rgb(250, 179, 135);
pub const COLOR_RED: Color = Color::Rgb(243, 139, 168);
pub const COLOR_MAUVE: Color = Color::Rgb(203, 166, 247);

pub fn usage_color(pct: f64) -> Color {
    if pct < 40.0 {
        COLOR_GREEN
    } else if pct < 70.0 {
        COLOR_YELLOW
    } else if pct < 90.0 {
        COLOR_PEACH
    } else {
        COLOR_RED
    }
}
