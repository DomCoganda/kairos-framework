use serde::{Serialize, Deserialize};
use crate::visual::Shadow;

/// Represents the thickness of a font from thin to black
#[derive(Serialize, Deserialize)]
pub enum FontWeight {
    Thin,
    ExtraLight,
    Light,
    Regular,
    Medium,
    Semibold,
    Bold,
    ExtraBold,
    Black,
}

/// Defines the slant style of text, either normal, italic or oblique
#[derive(Serialize, Deserialize)]
pub enum FontStyle {
    Italic,
    Normal,
    Oblique,
}

/// Represents text decoration such as underline or strikethrough
#[derive(Serialize, Deserialize)]
pub enum TextDecoration {
    Underline,
    Strikethrough,
    Overline,
}

/// Represents a complete font definition used inside SeraphOS
#[derive(Serialize, Deserialize)]
pub struct Font {
    pub size: f32,
    pub weight: FontWeight,
    pub style: FontStyle,
    pub decoration: Option<TextDecoration>,
    pub shadow: Option<Shadow>,
}

/// Predefined font styles for common use across all SeraphOS applications
#[derive(Serialize, Deserialize)]
pub struct Typography {
    pub family: String,
    pub mono_family: String,
    pub h1: Font,
    pub h2: Font,
    pub h3: Font,
    pub h4: Font,
    pub body: Font,
    pub caption: Font,
    pub label: Font,
    pub code: Font,
}

#[derive(Clone)]
pub enum TextStyle {
    H1,
    H2,
    H3,
    H4,
    Body,
    Caption,
    Label,
    Code,
}