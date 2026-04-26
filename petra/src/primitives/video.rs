use crate::primitives::types::*;
use mantle::source::FileSource;

///Defines the structure of a video in seraphUi
pub struct Video {
    pub source: FileSource, 
    pub width: Space,
    pub height: Space,
    pub autoplay: bool,
    pub repeat: bool,
    pub muted: bool,
    pub controls_visibility: bool,
}