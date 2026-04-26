use kairos_macros::primitive;
use mantle::color::ColorSource;
use crate::primitives::types::HorizontalAlignment;
use mantle::typography::TextStyle;
use crate::Space;

///Defines how the text primitive handles over flow when it can not wrap to the next line

#[derive(Clone)]
pub enum Truncation {
    Character,
    Word,
    Ellipsis,
}

///Defines how text is structured in seraph ui

#[primitive]
#[derive(Clone)]
pub struct Text {
    pub content: String,
    pub style: TextStyle,
    pub color: Option<ColorSource>,
    pub alignment: Option<HorizontalAlignment>,
    pub max_lines: Option<u32>,
    pub truncation: Option<Truncation>,
    pub width: Option<Space>,
}

/// Make it so users can create a text component in SeraphUi
impl Text {
    pub fn new() -> Self {
        Text {
            content: String::new(),
            style: TextStyle::Body,
            color: None,
            alignment: None,
            max_lines: None,
            truncation: None,
            width: None,
        }
    }
}


impl From<&str> for Text {
    fn from(s: &str) -> Self {
        Text::new().content(s.to_string())
    }
}

impl From<String> for Text {
    fn from(s: String) -> Self {
        Text::new().content(s)
    }
}