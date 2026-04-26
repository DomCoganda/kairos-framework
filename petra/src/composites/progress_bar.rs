use kairos_macros::primitive;
use mantle::color::ColorSource;
use mantle::palette::PaletteColor::{Accent, Secondary};
use crate::primitives::types::Space;
use crate::Space::Fill;

#[primitive]
pub struct ProgressBar {
    pub value: f32,
    pub min: f32,
    pub max: f32,
    pub width: Space,
    pub height: f32,
    pub color: ColorSource,
    pub background: ColorSource,
}

impl ProgressBar {
    pub fn new() -> Self {
        ProgressBar {
            value: 0.0,
            min: 0.0,
            max: 1.0,
            width: Fill(1),
            height: 4.0,
            color: ColorSource::Palette(Accent),
            background: ColorSource::Palette(Secondary),
        }
    }
}