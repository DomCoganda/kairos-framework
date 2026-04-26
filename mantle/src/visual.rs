use serde::{Serialize, Deserialize};
use crate::color::{Color, ColorSource};
use crate::border::Border;

/// Defines how a texture blends with the surface beneath it
#[derive(Serialize, Deserialize, Clone)]
pub enum BlendMode {
    Multiply,
    Overlay,
    Screen,
    Normal,
}

/// A visual texture applied on top of a surface using a PNG image
#[derive(Serialize, Deserialize, Clone)]
pub struct Texture {
    pub path: String,
    pub opacity: f32,
    pub blend: BlendMode,
}

/// Defines how a shadow looks for text and components
#[derive(Serialize, Deserialize, Clone)]
pub struct Shadow {
    pub color: Color,
    pub offset_x: f32,
    pub offset_y: f32,
    pub blur: f32,
    pub spread: f32,
}

/// Defines visual effects applied to a surface including blur, tint, opacity, border, texture and shadow
#[derive(Serialize, Deserialize, Clone)]
pub struct VisualEffect {
    pub blur: f32,
    pub tint: Color,
    pub opacity: f32,
    pub border: Option<Border>,
    pub texture: Option<Texture>,
    pub shadow: Option<Shadow>,
}

/// Defines whether a surface is a solid colour or has visual effects such as blur and transparency
#[derive(Serialize, Deserialize, Clone)]
pub enum SurfaceStyle {
    Solid(ColorSource),
    Effect(VisualEffect),
}

/// Defines what type of cursor is shown over a component
#[derive(Serialize, Deserialize)]
pub enum Cursor {
    Default,
    Pointer,
    Text,
    Crosshair,
    Grab,
    Grabbing,
    NotAllowed,
}

impl From<Color> for SurfaceStyle {
    fn from(color: Color) -> Self {
        SurfaceStyle::Solid(ColorSource::Custom(color))
    }
}

impl From<ColorSource> for SurfaceStyle {
    fn from(source: ColorSource) -> Self {
        SurfaceStyle::Solid(source)
    }
}
