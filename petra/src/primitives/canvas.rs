use crate::primitives:: types:: Space;
use mantle::palette::Fill as BackgroundFill;
use kairos_macros::primitive;

///Defines how a canvas is structured in seraph ui
#[primitive]
pub struct Canvas {
    pub width: Space,
    pub height: Space,
    pub background: Option<BackgroundFill>,
}

impl Canvas {
    pub fn new() -> Self {
        Canvas {
            width: Space::Shrink,
            height: Space::Shrink,
            background: None,
        }
    }
}