use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub enum Shape {
    Dot,
    Line,
    Arc,
    Triangle,
    Square,
    Custom(String),
}

///Defines the direction a icon can spin in seraphUi
#[derive(Clone)]
pub enum Direction {
    Forward,
    Reverse,
}

///Defines the structure of a icon in seraphUi
#[derive(Clone)]
pub enum SpinnerMotion {
    Ring,
    Chase,
    Spin,
    Wave,
    Grow,
    Bounce,
    Orbit,
    Pulse,
    Morph(Vec<Shape>),
    Comet,
    Ripple,
}
