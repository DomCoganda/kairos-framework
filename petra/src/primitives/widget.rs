use std::collections::HashMap;
use crate::composites::avatar::Avatar;
use crate::{Binding, Button, Canvas, Checkbox, Column, Divider, Graph, Icon, InputEvent, Radio, Row, Slider, Spacer, Stack, Text, Toggle};
use crate::composites::dropdown::Dropdown;
use crate::composites::progress_bar::ProgressBar;
use crate::primitives::positioned::Positioned;
use crate::primitives::scrubber::Scrubber;
use crate::primitives::spinner::Spinner;
use crate::primitives::text_input::TextInput;
use crate::primitives::image::Image;

#[derive(Clone)]
pub enum Widget {
    Row(Row),
    Column(Column),
    Canvas(Canvas),
    Button(Button),
    Text(Text),
    Spacer(Spacer),
    Graph(Graph),
    Icon(Icon),
    Divider(Divider),
    Avatar(Avatar),
    TextInput(TextInput),
    Scrollable(Box<Widget>),
    Stack(Stack),
    Spinner(Spinner),
    Positioned(Positioned),
    ProgressBar(ProgressBar),
    Scrubber(Scrubber),
    Dropdown(Dropdown),
    Checkbox(Checkbox),
    Toggle(Toggle),
    Slider(Slider),
    Radio(Radio),
    Image(Image),
    Deferred(std::sync::Arc<dyn Fn(&mantle::palette::Palette) -> Widget + Send + Sync>),
    Clickable(Box<Widget>, HashMap<Binding, std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(InputEvent) + Send>>>>),
    /*
    Checkbox,
    Overlay,
    Padding,
    Radio,
    Slider,
    Toast,
    Toggle,
    Video,
    */
}