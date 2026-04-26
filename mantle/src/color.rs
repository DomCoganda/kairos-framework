use serde::{Serialize, Deserialize};
pub use crate::palette::PaletteColor;

/// Seraph Theme representation of a color with a hex value and alpha transparency
#[derive(Serialize, Deserialize, Clone)]
pub struct Color {
    pub hex: String,
    pub alpha: f32,
}

/// Defines whether an icon uses a palette colour or a custom colour
#[derive(Serialize, Deserialize, Clone)]
pub enum ColorSource  {
    Palette(PaletteColor),
    Custom(Color),
}

impl Color {
    ///Makes it easier to create a new color 
    pub fn new(hex: &str, alpha: f32) -> Color {
        Color {
            hex: hex.to_string(),
            alpha,
        }
    }

    ///Converts Seraph Theme hex colors to rgba valuse
    pub fn to_rgba (&self) -> (f32,f32,f32,f32) {
        let r: f32 = u8::from_str_radix(&self.hex[1..3],16).unwrap_or(0) as f32 / 255.0;
        let g: f32 = u8::from_str_radix(&self.hex[3..5],16).unwrap_or(0) as f32 / 255.0;
        let b: f32 = u8::from_str_radix(&self.hex[5..7],16).unwrap_or(0) as f32 / 255.0;
        (r, g, b, self.alpha)
    }

    pub fn with_hue_shift(&self, degrees: f32) -> Color {
        let (r, g, b, a) = self.to_rgba();
        let (h, s, l) = rgb_to_hsl(r, g, b);
        let new_h = (h + degrees / 360.0).rem_euclid(1.0);
        let (nr, ng, nb) = hsl_to_rgb(new_h, s, l);
        Color {
            hex: format!("#{:02X}{:02X}{:02X}",
                         (nr * 255.0) as u8,
                         (ng * 255.0) as u8,
                         (nb * 255.0) as u8),
            alpha: a,
        }
    }

    pub fn with_alpha(&self, alpha: f32) -> Color {
        Color { hex: self.hex.clone(), alpha }
    }
}

impl From<Color> for ColorSource {
    fn from(value:Color) -> Self {
        ColorSource::Custom(value)
    }
}

impl From<PaletteColor> for ColorSource {
    fn from(value: PaletteColor) -> Self {
        ColorSource::Palette(value)
    }
}


fn rgb_to_hsl(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let l = (max + min) / 2.0;
    if max == min {
        return (0.0, 0.0, l);
    }
    let d = max - min;
    let s = if l > 0.5 { d / (2.0 - max - min) } else { d / (max + min) };
    let h = if max == r {
        (g - b) / d + if g < b { 6.0 } else { 0.0 }
    } else if max == g {
        (b - r) / d + 2.0
    } else {
        (r - g) / d + 4.0
    } / 6.0;
    (h, s, l)
}

fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (f32, f32, f32) {
    if s == 0.0 {
        return (l, l, l);
    }
    let q = if l < 0.5 { l * (1.0 + s) } else { l + s - l * s };
    let p = 2.0 * l - q;
    let r = hue_to_rgb(p, q, h + 1.0 / 3.0);
    let g = hue_to_rgb(p, q, h);
    let b = hue_to_rgb(p, q, h - 1.0 / 3.0);
    (r, g, b)
}

fn hue_to_rgb(p: f32, q: f32, t: f32) -> f32 {
    let t = t.rem_euclid(1.0);
    if t < 1.0/6.0 { return p + (q - p) * 6.0 * t; }
    if t < 1.0/2.0 { return q; }
    if t < 2.0/3.0 { return p + (q - p) * (2.0/3.0 - t) * 6.0; }
    p
}