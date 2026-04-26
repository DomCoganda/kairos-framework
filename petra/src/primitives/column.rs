use kairos_macros::primitive;
use mantle::sizing::Size;
use crate::primitives::types::*;
use crate::primitives::padding::Padding;
use crate::primitives::types::Space::{Fill, Shrink};
use crate::primitives::widget::Widget;
use mantle::palette::Fill as BackgroundFill;
use mantle::scrollbar::ScrollbarStyle;

///Defines the structure of a column in seraphUi

#[primitive]
pub struct Column {
    pub width: Space,
    pub height: Space,
    pub gap: Space,
    pub padding: Padding,
    pub alignment: HorizontalAlignment,
    pub background: Option<BackgroundFill>,
    pub children: Vec<Widget>,
    pub scrollable: bool,
    pub scrollbar: ScrollbarStyle
}

impl Column {
    pub fn new() -> Self {
        Column {
            width: Shrink,
            height:Fill(1),
            gap: Space::Scale(Size::Sm),
            padding: Padding::all(Space::Scale(Size::Sm)),
            alignment: HorizontalAlignment::Center,
            background: None,
            children: vec![],
            scrollable: true,
            scrollbar: ScrollbarStyle::hidden(),
        }
    }
}