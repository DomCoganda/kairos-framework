use crate::color::ColorSource;
use crate::palette::PaletteColor;

#[derive(Clone)]
pub struct ScrollbarStyle {
    pub width: f32,
    pub track_color: ColorSource,
    pub thumb_color: ColorSource,
    pub thumb_radius: f32,
    pub visible: bool,
}

impl ScrollbarStyle {
    pub fn hidden() -> Self {
        ScrollbarStyle {
            width: 0.0,
            track_color: ColorSource::Palette(PaletteColor::Background),
            thumb_color: ColorSource::Palette(PaletteColor::Secondary),
            thumb_radius: 0.0,
            visible: false,
        }
    }

    pub fn default() -> Self {
        ScrollbarStyle {
            width: 8.0,
            track_color: ColorSource::Palette(PaletteColor::Background),
            thumb_color: ColorSource::Palette(PaletteColor::Secondary),
            thumb_radius: 4.0,
            visible: true,
        }
    }
}