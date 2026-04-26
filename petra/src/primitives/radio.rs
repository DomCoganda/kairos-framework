use kairos_macros::primitive;
use mantle::palette::Fill;
use mantle::color::ColorSource;
use mantle::palette::PaletteColor::{Accent, Secondary};
use nuntius::signal::Signal;
use crate::primitives::text::Text;

#[derive(Clone)]
pub struct RadioStyle {
    pub selected: Fill,
    pub unselected: Fill,
    pub dot: ColorSource,
}

#[primitive]
pub struct Radio {
    pub value: String,
    pub selected: Signal<String>,
    pub label: Option<Text>,
    pub on_select: Option<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String) + Send>>>>,
    pub style: RadioStyle,
}

impl Radio {
    pub fn new() -> Self {
        Radio {
            value: String::new(),
            selected: Signal::new(String::new()),
            label: None,
            on_select: None,
            style: RadioStyle {
                selected: Fill::Palette(Accent),
                unselected: Fill::Palette(Secondary),
                dot: ColorSource::Palette(mantle::palette::PaletteColor::Text),
            },
        }
    }
}