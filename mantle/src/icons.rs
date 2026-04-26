use serde::{Serialize, Deserialize};
use crate::color::{ ColorSource};
use crate::sizing::Size;
use crate::source::FileSource;

/// Defines whether an icon is filled or outlined
#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum IconStyle {
    Filled,
    Outline,
}

///Defines the structure where the icon source is located
#[derive(Serialize, Deserialize, Clone)]
pub enum IconSource {
    System(String),
    Path(FileSource),
    Raw(String),
}


/// Defines the default appearance of icons across the types
#[derive(Serialize, Deserialize)]
pub struct IconTheme {
    pub style: IconStyle,
    pub color: ColorSource,
    pub size: Size,
}