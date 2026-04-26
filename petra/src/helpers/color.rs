use mantle::color::Color;
use mantle::color::ColorSource;

pub fn hex(hex: &str, alpha: f32) -> ColorSource {
    let color = Color::new(hex, alpha);
    ColorSource::Custom(color)
}