use kairos_macros::primitive;
use mantle::color::ColorSource;
use mantle::sizing::Size;

#[primitive]
pub struct Avatar {
    pub name: String,
    pub size: Size,
    pub color: Option<ColorSource>,
}

impl Avatar {
    pub fn new() -> Self {
        Avatar {
            name: String::new(),
            size: Size::Md,
            color: None,
        }
    }
}