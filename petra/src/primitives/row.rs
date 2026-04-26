use kairos_macros::primitive;
use mantle::sizing::Size;
use crate::primitives::types::*;
use crate::primitives::padding::Padding;
use crate::primitives::types::Space::{Fill, Shrink};
use crate::primitives::widget::Widget;
use mantle::palette::Fill as BackgroundFill;

///Defines the structure of a row in seraphUi

#[primitive]
pub struct Row {
    pub width: Space,
    pub height: Space,
    pub gap: Space,
    pub padding: Padding,
    pub wrap: bool,
    pub alignment: VerticalAlignment,
    pub background: Option<BackgroundFill>,
    pub children: Vec<Widget>
}

impl Row {
    pub fn new() -> Self {
        Row {
            width: Fill(1),
            height:Shrink,
            gap: Space::Scale(Size::Sm),
            padding: Padding::all(Space::Scale(Size::Sm)),
            wrap: false,
            alignment: VerticalAlignment::Center,
            background: None,
            children: vec![],
        }
    }
}