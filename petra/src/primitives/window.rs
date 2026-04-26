use mantle::types::theme::{ThemeSet, ThemeVariant};
use crate::primitives::widget::Widget;

pub struct Window{
    pub theme: ThemeSet,
    pub theme_name: ThemeVariant,
    pub children: Vec<Widget>,
    pub title: String,
}

impl Window{
    pub fn new(title: &str) -> Self{
        Window {
            theme: ThemeSet::default(),
            theme_name: ThemeVariant::Light,
            children: vec![],
            title: title.into(),
        }
    }

    pub fn theme(mut self, theme: ThemeSet) -> Self {
        self.theme = theme;
        self
    }

    pub fn mode(mut self, theme_name: ThemeVariant) -> Self {
        self.theme_name = theme_name;
        self
    }

    pub fn children(mut self, children: Vec<Widget>) -> Self {
        self.children = children;
        self
    }
}