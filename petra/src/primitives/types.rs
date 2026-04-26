use mantle::sizing::Size;
use crate::primitives::text::Text;
use crate::{Icon, Widget};

///Defines the type of spacing in SeraphUi
#[derive(Copy, Clone)]
pub enum Space {
    Fill(u16),
    Shrink,
    Custom(f32),
    Scale(Size),
}

///Defines Horizontal alignment options for seraphUi
#[derive(Clone)]
pub enum HorizontalAlignment {
    Left,
    Center,
    Right,
    Stretch
}

///Defines Vertical alignment options for seraphUi
#[derive(Clone)]
pub enum VerticalAlignment {
    Top,
    Center,
    Bottom,
    Stretch
}

///Defines weather should be oriented horizontally or vertically
#[derive(Clone)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

///Defines the different position of where a component can live in seraphUi
pub enum Position {
    TopLeft,
    TopCenter,
    TopRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
}

///Defines the structure of labels inside seraphUi
#[derive( Clone)]
pub enum LabelContent {
    Label(Text),
    Icon(Box<Widget>),
    Both(Text, Box<Widget>),
}

impl From<Widget> for LabelContent {
    fn from(w: Widget) -> Self {
        LabelContent::Icon(Box::new(w))
    }
}

///Allows a string literal to be used as a LabelContent in SeraphUi
impl From<&str> for LabelContent {
    fn from(label: &str) -> Self {
        LabelContent::Label(Text::new().content(label.to_string()))
    }
}

///Allows a Text to be used as a LabelContent in SeraphUi
impl From<Text> for LabelContent {
    fn from(label: Text) -> Self {
        LabelContent::Label(label)
    }
}

///Allows an Icon to be used as a LabelContent in SeraphUi
impl From<Icon> for LabelContent {
    fn from(icon: Icon) -> Self {
        LabelContent::Icon(Box::new(Widget::Icon(icon)))
    }
}