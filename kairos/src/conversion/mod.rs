mod text;
mod button;
mod row;
mod column;
mod canvas;
mod style;
mod spacer;
mod graph;
mod clickable;
mod scrollable;
mod icon;
mod divider;
mod avatar;
mod text_input;
mod stack;
pub mod positioned;
pub mod spinner;
mod toggle;
mod checkbox;
mod slider;
mod radio;
mod progress_bar;
mod scrubber;
mod dropdown;
mod image;

use mantle::types::theme::{ThemeSet, ThemeVariant};
use petra::{InputEvent, Signal, Widget};

#[allow(dead_code)]
pub(crate) enum KairosEvent {
    Press(std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() + Send>>>),
    StringInput(Signal<String>, String),
    Tick,
    None,
    InputPress(std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(InputEvent) + Send>>>, f32, f32),
    KeyPress(String),
}

impl std::fmt::Debug for KairosEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KairosEvent::StringInput(_, _) => write!(f, "StringInput(...)"),
            KairosEvent::None => write!(f, "None"),
            KairosEvent::Press(_) => write!(f, "Press(...)"),
            KairosEvent::Tick => write!(f, "Tick"),
            KairosEvent::InputPress(_, _, _) => write!(f, "InputPress(...)"),
            KairosEvent::KeyPress(k) => write!(f, "KeyPress({})", k),
        }
    }
}

impl Clone for KairosEvent {
    fn clone(&self) -> Self {
        match self {
            KairosEvent::StringInput(sig, val) => KairosEvent::StringInput(sig.clone(), val.clone()),
            KairosEvent::None => KairosEvent::None,
            KairosEvent::Press(f) => KairosEvent::Press(std::sync::Arc::clone(f)),
            KairosEvent::Tick => KairosEvent::Tick,
            KairosEvent::InputPress(f, x, y) => KairosEvent::InputPress(std::sync::Arc::clone(f), *x, *y),
            KairosEvent::KeyPress(k) => KairosEvent::KeyPress(k.clone()),
        }
    }
}

pub(crate) fn to_iced(widget: Widget, theme: &ThemeSet, variant: ThemeVariant, time: f32) -> iced::Element<'static, KairosEvent> {
    let palette = match variant {
        ThemeVariant::Dark => &theme.dark.palette,
        ThemeVariant::Light => &theme.light.palette,
    };
    match widget {
        Widget::Text(t) => text::convert_text(t, theme, palette),
        Widget::Row(r) => row::convert_row(r, theme, variant, time),
        Widget::Column(c) => column::convert_column(c, theme, variant, time),
        Widget::Button(b) => button::convert_button(b, palette, theme, variant, time),
        Widget::Canvas(c) => canvas::convert_canvas(c, palette, theme),
        Widget::Spacer(s) => spacer::convert_spacer(s, theme, palette),
        Widget::Graph(g) => graph::convert_graph(g, palette, theme),
        Widget::Deferred(d) => to_iced((d)(palette), theme, variant, time),
        Widget::Clickable(inner, bindings) => clickable::convert_clickable(inner, bindings, theme, variant, time),
        Widget::Scrollable(inner) => scrollable::convert_scrollable(inner, theme, variant, time),
        Widget::Icon(i) => icon::icon_to_iced(i, theme, palette),
        Widget::Divider(d) => divider::convert_divider(d, palette, theme),
        Widget::Avatar(a) => avatar::convert_avatar(a, theme, palette),
        Widget::TextInput(input) => text_input::convert_text_input(input, palette, theme, variant),
        Widget::Stack(s) => stack::convert_stack(s, theme, variant, time),
        Widget::Positioned(p) => positioned::convert_positioned(p, theme, variant, time),
        Widget::Spinner(s) => spinner::convert_spinner(s, palette, theme, time),
        Widget::Toggle(t) => toggle::convert_toggle(t, palette, theme),
        Widget::Checkbox(c) => checkbox::convert_checkbox(c, palette, theme),
        Widget::Slider(s) => slider::convert_slider(s, palette, theme),
        Widget::Radio(r) => radio::convert_radio(r, palette, theme),
        Widget::ProgressBar(p) => progress_bar::convert_progress_bar(p, palette, theme),
        Widget::Scrubber(s) => scrubber::convert_scrubber(s, palette, theme),
        Widget::Dropdown(d) => dropdown::convert_dropdown(d, palette, theme),
        Widget::Image(img) => image::convert_image(img, palette, theme),
    }
}