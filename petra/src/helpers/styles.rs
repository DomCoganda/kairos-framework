use kairos_macros::component;
use serde::{Serialize, Deserialize};
use mantle::border::Border;
use mantle::palette::{Fill, PaletteColor::Primary};
use mantle::color::{ColorSource, PaletteColor::Text};
use mantle::visual::Shadow;


#[component]
#[export_macro]
#[derive(Serialize, Deserialize, Clone)]
pub struct WidgetStyle {
    pub background: Fill,
    pub text_color: ColorSource,
    pub border: Border,
    pub shadow: Option<Shadow>,
}



impl WidgetStyle {
    pub fn new() -> Self {
        WidgetStyle {
            background: Fill::Palette(Primary),
            text_color: ColorSource::Palette(Text),
            border: Border::new(),
            shadow: None,
        }
    }
}