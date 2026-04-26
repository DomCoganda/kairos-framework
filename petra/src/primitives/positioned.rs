use crate::{Space, Widget};
use crate::primitives::spacer::Spacer;

#[derive(Clone)]
pub struct Positioned {
    pub x: f32,
    pub y: f32,
    pub width: Space,
    pub height: Space,
    pub child: Box<Widget>,
}

impl Positioned {
    pub fn new() -> Self {
        Positioned {
            x: 0.0,
            y: 0.0,
            width: Space::Shrink,
            height: Space::Shrink,
            child: Box::new(Widget::Spacer(Spacer::new())),
        }
    }
    pub fn x(mut self, value: f32) -> Self { self.x = value; self }
    pub fn y(mut self, value: f32) -> Self { self.y = value; self }
    pub fn width(mut self, value: Space) -> Self { self.width = value; self }
    pub fn height(mut self, value: Space) -> Self { self.height = value; self }
    pub fn child(mut self, value: Box<Widget>) -> Self { self.child = value; self }
}