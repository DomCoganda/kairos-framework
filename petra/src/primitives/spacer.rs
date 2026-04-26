use kairos_macros::primitive;
use crate::primitives::types::{Orientation, Space};
use mantle::visual::SurfaceStyle;

///Defines the structure of the spacer in SeraphUi

#[primitive]
pub struct Spacer {
    pub size: Space,
    pub orientation: Orientation,
    pub background: Option<SurfaceStyle>
}

impl Spacer {
    pub fn new() -> Self {
        Spacer{
            size: Space::Shrink,
            orientation: Orientation::Vertical,
            background: None,
        }
    }
}