use kairos_macros::primitive;
use mantle::color::ColorSource;
use mantle::palette::PaletteColor::{Accent, Secondary};
use nuntius::signal::Signal;
use crate::primitives::types::Space;
use crate::Space::Fill;

#[primitive]
pub struct Scrubber {
    pub current_frame: Signal<u32>,
    pub total_frames: u32,
    pub in_point: Option<Signal<u32>>,
    pub out_point: Option<Signal<u32>>,
    pub width: Space,
    pub height: f32,
    pub track_color: ColorSource,
    pub fill_color: ColorSource,
    pub handle_color: ColorSource,
    pub on_seek: Option<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(u32) + Send>>>>,
}

impl Scrubber {
    pub fn new() -> Self {
        Scrubber {
            current_frame: Signal::new(0),
            total_frames: 0,
            in_point: None,
            out_point: None,
            width: Fill(1),
            height: 4.0,
            track_color: ColorSource::Palette(Secondary),
            fill_color: ColorSource::Palette(Accent),
            handle_color: ColorSource::Palette(Accent),
            on_seek: None,
        }
    }
}