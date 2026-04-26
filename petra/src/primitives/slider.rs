use kairos_macros::primitive;
use mantle::color::ColorSource;
use mantle::palette::PaletteColor::{Accent, Secondary, Text};
use nuntius::signal::Signal;

#[derive(Clone)]
pub struct SliderStyle {
    pub track: ColorSource,
    pub fill: ColorSource,
    pub handle: ColorSource,
}

#[primitive]
pub struct Slider {
    pub min: f32,
    pub max: f32,
    pub step: f32,
    pub value: Signal<f32>,
    pub on_change: Option<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f32) + Send>>>>,
    pub style: SliderStyle,
}

impl Slider {
    pub fn new() -> Self {
        Slider {
            min: 0.0,
            max: 1.0,
            step: 0.01,
            value: Signal::new(0.0),
            on_change: None,
            style: SliderStyle {
                track: ColorSource::Palette(Secondary),
                fill: ColorSource::Palette(Accent),
                handle: ColorSource::Palette(Text),
            },
        }
    }
}