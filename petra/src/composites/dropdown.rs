use kairos_macros::primitive;
use nuntius::signal::Signal;
use crate::primitives::types::Space;
use crate::Space::Fill;

#[primitive]
#[export_macro]
pub struct Dropdown {
    pub options: Vec<String>,
    pub selected: Signal<Option<String>>,
    pub placeholder: String,
    pub width: Space,
    pub open: Signal<bool>,
}

impl Dropdown {
    pub fn new() -> Self {
        Dropdown {
            options: vec![],
            selected: Signal::new(None),
            placeholder: "Select...".to_string(),
            width: Fill(1),
            open: Signal::new(false),
        }
    }
}
