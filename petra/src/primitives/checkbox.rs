use kairos_macros::primitive;
use mantle::palette::Fill;
use mantle::color::ColorSource;
use nuntius::signal::Signal;
use crate::primitives::text::Text;

#[derive(Clone)]
pub struct CheckboxStyle {
    pub background: Fill,
    pub checkmark: ColorSource,
}

#[derive(Clone)]
pub struct CheckboxStyles {
    pub checked: CheckboxStyle,
    pub unchecked: CheckboxStyle,
}

#[primitive]
pub struct Checkbox {
    pub state: Signal<bool>,
    pub label: Option<Text>,
    pub on_change: Option<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(bool) + Send>>>>,
    pub style: CheckboxStyles,
}

impl Checkbox {
    pub fn new() -> Self {
        Checkbox {
            state: Signal::new(false),
            label: None,
            on_change: None,
            style: CheckboxStyles {
                checked: CheckboxStyle {
                    background: Fill::Palette(mantle::palette::PaletteColor::Accent),
                    checkmark: ColorSource::Palette(mantle::palette::PaletteColor::Text),
                },
                unchecked: CheckboxStyle {
                    background: Fill::Palette(mantle::palette::PaletteColor::Secondary),
                    checkmark: ColorSource::Palette(mantle::palette::PaletteColor::Text),
                },
            },
        }
    }
}