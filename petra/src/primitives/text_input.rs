use kairos_macros::primitive;
use nuntius::signal::Signal;
use crate::primitives::text::Text;
use crate::primitives::types::Space;
use crate::{Widget, WidgetStyle};
use crate::Space::Fill;

///Defines the structure of the prefix and suffix of the text input
#[derive(Clone)]
pub enum InputEnds {
    Icon(Box<Widget>),
    Text(Text),
}

///Defines the structure of a text input in seraphUi
#[primitive]
pub struct TextInput{
    pub width: Space,
    pub height: Space,
    pub label: Option<Text>,
    pub prefix: Option<InputEnds>,
    pub placeholder: Option<Text>,
    pub value: Signal<String>,
    pub suffix: Option<InputEnds>,
    pub error: Option<Text>,
    pub style: WidgetStyle

}

impl TextInput{
    pub fn new() -> Self{
        TextInput{
            width: Fill(1),
            height: Fill(1),
            label: None,
            prefix: None,
            placeholder: None,
            value: Signal::new(String::new()),
            suffix: None,
            error: None,
            style: WidgetStyle::new(),
        }
    }
}