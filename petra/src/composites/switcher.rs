use mantle::palette::Palette;
use nuntius::signal::Signal;
use crate::{Component, Widget};

pub struct Switcher<T> {
    pub signal: Signal<T>,
    pub view: std::sync::Arc<dyn Fn(T) -> Widget + Send + Sync>,
}

impl<T> Switcher<T> {
    pub fn new(signal: Signal<T>, view: std::sync::Arc<dyn Fn(T) -> Widget + Send + Sync>) -> Self {
        Switcher { signal, view }
    }
}

impl<T: Copy> Component for Switcher<T> {
    fn build(&self, _palette: &Palette) -> Widget {
        let value = self.signal.get();
        (self.view)(value)
    }
}

impl<T: Clone> Clone for Switcher<T> {
    fn clone(&self) -> Self {
        Switcher {
            signal: self.signal.clone(),
            view: std::sync::Arc::clone(&self.view),
        }
    }
}

