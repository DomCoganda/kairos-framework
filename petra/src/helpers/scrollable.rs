// scrollable.rs
use crate::Widget;

pub fn scrollable(child: Widget) -> Widget {
    Widget::Scrollable(Box::new(child))
}