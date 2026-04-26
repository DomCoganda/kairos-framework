use kairos_macros::primitive;
use crate::primitives::types::Space;
use mantle::source::FileSource;


/// Defines how an image is fitted within its bounds
#[derive(Clone, Debug, PartialEq)]
pub enum ImageFit {
    Contain,
    Cover,
    Fill,
    None
}

impl Default for ImageFit {
    fn default() -> Self {
        ImageFit::Contain
    }
}

/// Displays a raster image from disk or embedded bytes
#[primitive]
pub struct Image {
    pub source: FileSource,
    pub width: Space,
    pub height: Space,
    pub fit: ImageFit,
}

impl Image {
    pub fn new() -> Self {
        Image {
            source: FileSource::File(String::new()),
            width: Space::Fill(1),
            height: Space::Fill(1),
            fit: ImageFit::default(),
        }
    }
}