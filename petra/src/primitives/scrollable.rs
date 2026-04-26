use mantle::color::ColorSource;
use mantle::line::LineStyle;
use crate::primitives::types::Orientation;

///Defines how (if) the scrollbar is visible in seraphUi
pub enum ScrollbarVisibility {
    Always,
    Hidden,
    OnScroll,
}

///Defines the structure of scrollable component in seraphUi
pub struct Scrollable {
    pub orientation: Orientation,
    pub scrollbar: ScrollbarVisibility,
    pub thickness: Option<f32>,
    pub style: Option<LineStyle>,
    pub color: Option<ColorSource>
}