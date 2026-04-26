use mantle::color::{Color, ColorSource};
use mantle::types::theme::ThemeSet;
use petra::{AxisPadding, Padding, Space};

pub fn hex_alpha(hex: &str, alpha: f32) -> Color {
    Color::new(hex, alpha)
}

pub fn rgba(r: f32, g: f32, b: f32, a: f32) -> ColorSource {
    let hex = format!("#{:02X}{:02X}{:02X}", (r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8);
    ColorSource::Custom(Color::new(&hex, a))
}

pub fn default() -> ThemeSet {
    ThemeSet::default()
}

pub fn horizontal(axis: AxisPadding) -> Padding {
    Padding::horizontal(axis)
}

pub fn vertical(axis: AxisPadding) -> Padding {
    Padding::vertical(axis)
}

pub fn symmetrical(value: Space) -> AxisPadding {
    AxisPadding::Symmetrical(value)
}

pub fn symmetric(horizontal: AxisPadding, vertical: AxisPadding) -> Padding {
    Padding::symmetric(horizontal, vertical)
}