use serde::{Serialize, Deserialize};
use crate::color::{Color, ColorSource};
use crate::visual::SurfaceStyle;

/// The full color palette used across a types
#[derive(Serialize, Deserialize)]
pub struct Palette {
    pub background: Color,
    pub surface: SurfaceStyle,
    pub primary: Color,
    pub secondary: Color,
    pub text: Color,
    pub accent: Color,
    pub error: Color,
    pub warning: Color,
    pub success: Color,
}

/// Enum that allows you to reference a colour from the palette by name
#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum PaletteColor {
    Primary,
    Secondary,
    Accent,
    Text,
    Background,
    Error,
    Warning,
    Success,
}

/// Enum that allows a component to use either the palette surface or a custom surface
#[derive(Serialize, Deserialize)]
#[derive(Clone)]
pub enum Fill {
    Surface,
    Palette(PaletteColor),
    Custom(SurfaceStyle),
}

impl From<PaletteColor> for Fill {
    fn from(palette: PaletteColor) -> Self {
        Fill::Palette(palette)
    }
}
impl From<SurfaceStyle> for Fill {
    fn from(s: SurfaceStyle) -> Self { Fill::Custom(s) }
}
impl From<crate::color::ColorSource> for Fill {
    fn from(c: crate::color::ColorSource) -> Self {
        Fill::Custom(SurfaceStyle::Solid(c))
    }
}

impl Palette {
    pub fn resolve<'a>(&'a self, source: &'a ColorSource) -> &'a str {
        match source {
            ColorSource::Custom(color) => &color.hex,
            ColorSource::Palette(pc) => match pc {
                PaletteColor::Primary => &self.primary.hex,
                PaletteColor::Secondary => &self.secondary.hex,
                PaletteColor::Accent => &self.accent.hex,
                PaletteColor::Text => &self.text.hex,
                PaletteColor::Background => &self.background.hex,
                PaletteColor::Error => &self.error.hex,
                PaletteColor::Warning => &self.warning.hex,
                PaletteColor::Success => &self.success.hex,
            }
        }
    }
}