use kairos_macros::primitive;
use mantle::palette::Fill;
use mantle::color::ColorSource;
use nuntius::signal::Signal;

#[derive(Clone)]
pub struct ToggleStyle {
    pub background: Fill,
    pub handle: ColorSource,
}

#[derive(Clone)]
pub struct ToggleState {
    pub on: ToggleStyle,
    pub off: ToggleStyle,
}

#[primitive]
pub struct Toggle {
    pub state: Signal<bool>,
    pub on_change: Option<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(bool) + Send>>>>,
    pub style: ToggleState,
}

impl Toggle {
    pub fn new() -> Self {
        Toggle {
            state: Signal::new(false),
            on_change: None,
            style: ToggleState {
                on: ToggleStyle {
                    background: Fill::Palette(mantle::palette::PaletteColor::Accent),
                    handle: ColorSource::Palette(mantle::palette::PaletteColor::Text),
                },
                off: ToggleStyle {
                    background: Fill::Palette(mantle::palette::PaletteColor::Secondary),
                    handle: ColorSource::Palette(mantle::palette::PaletteColor::Text),
                },
            },
        }
    }
}