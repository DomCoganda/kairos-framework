
#[derive(Hash, Eq, PartialEq,Clone)]
pub struct Key(pub String);

#[derive(Hash, Eq, PartialEq,Clone)]
pub enum MouseButton {
    Left,
    Right,
    Middle
}


#[derive(Hash, Eq, PartialEq,Clone)]
pub enum Binding {
    Mouse(MouseButton),
    Key(Key),
}

#[derive(Clone)]
pub enum InputEvent {
    Click { x: f32, y: f32 },
    KeyPress(Key),
}