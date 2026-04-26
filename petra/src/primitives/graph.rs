use kairos_macros::primitive;
use mantle::color::ColorSource;
use mantle::palette::PaletteColor::{Accent, Background, Secondary};
use crate::Space;


#[primitive]
pub struct Graph{
    pub data: Vec<f32>,
    pub width: Space,
    pub height: Space,
    pub color: ColorSource,
    pub fill_color: ColorSource,
    pub fill_opacity: f32,
    pub min: f32,
    pub max: f32,
    pub line_width: f32,
    pub bg_color: ColorSource,
    pub bg_opacity: f32,
}

impl Graph {
    pub fn new() -> Self {
        Graph{
            data: vec![],
            width: Space::Shrink,
            height: Space::Shrink,
            color: ColorSource::Palette(Accent),
            fill_color: ColorSource::Palette(Secondary),
            fill_opacity: 0.5,
            min: 0.0,
            max: 0.0,
            line_width: 1.5,
            bg_color: ColorSource::Palette(Background),
            bg_opacity: 1.0,
        }
    }
}