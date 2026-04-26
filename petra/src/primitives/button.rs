use kairos_macros::primitive;
use crate::primitives::types::*;
use crate::{Padding, Text};
use crate::helpers::styles::WidgetStyle;

///Defines the structure of a button in seraphUi

#[primitive]
pub struct Button {
    pub label: LabelContent,
    pub width: Space,
    pub height: Space,
    #[nested]
    pub style: WidgetStyle,
    pub on_press: Option<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() + Send>>>>,
    pub padding: Padding,
}



///Makes it so user can create a button component in SeraphUI
impl Button {
    pub fn new() -> Self {
        Button {
            label: LabelContent::Label(Text::new().content(" ".to_string())),
            width: Space::Shrink,
            height: Space::Shrink,
            style: WidgetStyle::new(),
            on_press: None,
            padding: Padding::none(),
        }
    }
}
