use kairos_macros::primitive;
use mantle::color::ColorSource;
use mantle::line::LineStyle;
use crate::primitives::types::Orientation;

///Defines the structure of a divider in SeraphUi


#[primitive]
pub struct Divider {
    pub orientation: Orientation,
    pub thickness: f32,
    pub style: LineStyle,
    pub color: ColorSource,
}

impl Divider {
    pub fn new() -> Self {
        Divider {
            orientation: Orientation::Horizontal,
            thickness: 1.0,
            style: LineStyle::Solid,
            color: ColorSource::Palette(mantle::palette::PaletteColor::Secondary),
        }
    }
}