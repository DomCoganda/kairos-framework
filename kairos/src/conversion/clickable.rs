use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use mantle::types::theme::{ThemeSet, ThemeVariant};
use petra::{Binding, InputEvent, Key, MouseButton};
use crate::conversion::{to_iced, KairosEvent};

pub(crate) fn convert_clickable(
    inner: Box<petra::Widget>,
    on_press: HashMap<Binding, Arc<Mutex<Box<dyn FnMut(InputEvent) + Send>>>>,
    theme: &ThemeSet,
    variant: ThemeVariant,
    time: f32,
) -> iced::Element<'static, KairosEvent> {
    let content = to_iced(*inner, theme, variant, time);

    if let Ok(mut registry) = crate::key_handlers().lock() {
        for (binding, handler) in &on_press {
            if let Binding::Key(Key(k)) = binding {
                registry.insert(format!("Named({k})"), handler.clone());
                registry.insert(k.clone(), handler.clone());
            }
        }
    }

    let cursor_pos = Arc::new(Mutex::new((0.0f32, 0.0f32)));
    let cursor_pos_move = cursor_pos.clone();
    let cursor_pos_left = cursor_pos.clone();
    let cursor_pos_right = cursor_pos.clone();
    let cursor_pos_middle = cursor_pos.clone();
    let left_handler = on_press.get(&Binding::Mouse(MouseButton::Left)).cloned();
    let right_handler = on_press.get(&Binding::Mouse(MouseButton::Right)).cloned();
    let middle_handler = on_press.get(&Binding::Mouse(MouseButton::Middle)).cloned();

    let area = iced::widget::mouse_area(content)
        .on_move(move |point| {
            *cursor_pos_move.lock().unwrap() = (point.x, point.y);
            KairosEvent::None
        })
        .on_press({
            let (x, y) = *cursor_pos_left.lock().unwrap();
            match &left_handler {
                Some(handler) => KairosEvent::InputPress(handler.clone(), x, y),
                None => KairosEvent::None,
            }
        })
        .on_right_press({
            let (x, y) = *cursor_pos_right.lock().unwrap();
            match &right_handler {
                Some(handler) => KairosEvent::InputPress(handler.clone(), x, y),
                None => KairosEvent::None,
            }
        })
        .on_middle_press({
            let (x, y) = *cursor_pos_middle.lock().unwrap();
            match &middle_handler {
                Some(handler) => KairosEvent::InputPress(handler.clone(), x, y),
                None => KairosEvent::None,
            }
        });
    area.into()
}