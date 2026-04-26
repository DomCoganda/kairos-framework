use std::collections::HashMap;
use crate::{Binding, InputEvent, Widget};



pub fn clickable(widget: Box<Widget>, bindings: HashMap<Binding, Box<dyn FnMut(InputEvent) + Send + 'static>>) -> Widget {
    let wrapped = bindings
        .into_iter()
        .map(|(binding, handler)| (binding, std::sync::Arc::new(std::sync::Mutex::new(handler))))
        .collect();
    Widget::Clickable(widget, wrapped)
}