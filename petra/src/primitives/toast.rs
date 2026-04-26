use std::time::Duration;
use mantle::color::ColorSource;
use mantle::visual::SurfaceStyle;
use crate::primitives::text::Text;
use crate::primitives::types::Position;

///Defines the styles of a toast in SeraphUi
pub struct ToastStyle {
    pub background: SurfaceStyle,
    pub text: ColorSource,
}
///Defines the structure of a toast message in seraphUi
pub struct Toast{
    pub message: Text,
    pub duration: Duration,
    pub position: Position,
    pub style: ToastStyle
}