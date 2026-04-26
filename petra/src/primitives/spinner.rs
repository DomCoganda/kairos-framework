use kairos_macros::primitive;
use mantle::animation::Animation;
use mantle::color::ColorSource;
use mantle::color::PaletteColor::Accent;
use mantle::shape::{Direction, Shape, SpinnerMotion};
use mantle::sizing::Size;

#[primitive]
pub struct Spinner {
    pub shape: Shape,
    pub motion: SpinnerMotion,
    pub size: Size,
    pub color: ColorSource,
    pub speed: Animation,
    pub direction: Direction,
    pub count: u8,
}

impl Spinner {
    pub fn new() -> Self {
        Spinner{
            shape: Shape::Dot,
            motion: SpinnerMotion::Ring,
            size: Size::Lg,
            color: ColorSource::Palette(Accent),
            speed: Animation::Normal,
            direction: Direction::Forward,
            count: 1,
        }
    }
}