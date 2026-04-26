use kairos_macros::primitive;
use mantle::color::ColorSource;
use mantle::icons::{IconSource, IconStyle};
use mantle::sizing::Size;

#[primitive]
pub struct Icon {
    pub source: IconSource,
    pub size: Size,
    pub color: Option<ColorSource>,
    pub style: IconStyle,
    pub rotation: f32,
}

impl Icon {
    pub fn new() -> Self {
        Icon {
            source: IconSource::Raw(String::new()),
            size: Size::Md,
            color: None,
            style: IconStyle::Filled,
            rotation: 0.0,
        }
    }
}