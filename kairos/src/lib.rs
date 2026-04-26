pub mod rendering;
mod runtime;
mod conversion;
pub mod macros;
mod helpers;
mod theme;

pub use runtime::App;
pub use runtime::run;

pub use crate::PaletteFill::Palette as BackgroundFill;

pub use theme::ActiveTheme;

pub use petra::Widget;
pub use petra::Text;
pub use petra::Row;
pub use petra::Column;
pub use petra::Button;
pub use petra::Signal;
pub use petra::Padding;
pub use petra::Switcher;
pub use petra::AxisPadding;
pub use petra::Graph;
pub use petra::Canvas;
pub use petra::Divider;
pub use petra::Avatar;
pub use petra::TextInput;
pub use petra::Binding;
pub use petra::MouseButton;
pub use petra::Key;
pub use petra::InputEvent;
pub use petra::Stack;
pub use petra::WidgetStyle;
pub use petra::Positioned;
pub use petra::Spinner;
pub use petra::SpinnerMotion;
pub use petra::SignalBuffer;
pub use petra::Toggle;
pub use petra::Checkbox;
pub use petra::Slider;
pub use petra::Radio;
pub use petra::ProgressBar;
pub use petra::Scrubber;
pub use petra::Dropdown;
pub use petra::Image;
pub use petra::ImageFit;

pub use petra::Direction as SpinnerDirection;

pub use petra::row as row;
pub use petra::column as column;
pub use petra::button as button;
pub use petra::spacer as spacer;
pub use petra::text as text;
pub use petra::hex as hex;
pub use petra::canvas as canvas;
pub use petra::clickable as clickable;
pub use petra::graph as graph;
pub use petra::scrollable as scrollable;
pub use petra::divider as divider;
pub use petra::avatar as avatar;
pub use petra::text_input as text_input;
pub use petra::widget_style as widget_style;
pub use petra::stack as stack;
pub use petra::spinner as spinner;
pub use petra::icon as icon;
pub use petra::toggle as toggle;
pub use petra::checkbox as checkbox;
pub use petra::slider as slider;
pub use petra::radio as radio;
pub use petra::progress_bar as progress_bar;
pub use petra::scrubber as scrubber;
pub use petra::dropdown as dropdown;
pub use petra::image as image;

pub use petra::Component;
pub use petra::Buildable;

pub use petra::primitives::spacer::Spacer;
pub use petra::primitives::types::Space::{Fill, Shrink};
pub use petra::primitives::types::Space::Custom;
pub use petra::primitives::types::Space::Scale;
pub use petra::primitives::types::LabelContent;
pub use petra::primitives::types::Orientation;
pub use petra::primitives::types::Space;
pub use petra::primitives::icon::Icon;
pub use petra::primitives::types::HorizontalAlignment;
pub use petra::primitives::types::VerticalAlignment;
pub use petra::primitives::types::Position;
pub use petra::AxisPadding::Symmetrical;

pub use mantle::scrollbar as scrollbar_style;
pub use mantle::shape::Shape as SpinnerShape;
pub use mantle::palette::Fill as PaletteFill;

pub use mantle::types::theme::ThemeVariant::{Dark, Light};
pub use mantle::source::FileSource::{File, Url};

pub use mantle::types::theme::ThemeSet;
pub use mantle::types::theme::ThemeVariant;
pub use mantle::palette::Palette;
pub use mantle::typography::TextStyle;
pub use mantle::border::BorderRadius;
pub use mantle::source::FileSource;
pub use mantle::icons::IconSource;
pub use mantle::line::LineStyle;
pub use mantle::icons::IconStyle;
pub use mantle::palette::PaletteColor;
pub use mantle::color::ColorSource;
pub use mantle::color::Color;
pub use mantle::visual::SurfaceStyle;
pub use mantle::border::Border;
pub use mantle::border::BorderSides;
pub use mantle::sizing::Size;
pub use mantle::animation::Animation;
pub use mantle::palette::PaletteColor::{Primary, Accent, Background, Secondary};
pub use mantle::palette::PaletteColor::Text as TextColor;

pub use helpers::horizontal;
pub use helpers::symmetrical;
pub use helpers::hex_alpha;
pub use helpers::default;
pub use helpers::rgba;
pub use helpers::vertical;
pub use helpers::symmetric;

pub use crate::conversion::positioned;

pub use kairos_macros::component;
pub use kairos_macros::icon_png;

pub mod icons {
    pub const PLAY: &str = include_str!("assets/icon/play.svg");
    pub const DOT: &str = include_str!("assets/icon/dot.svg");
    pub const ARC: &str = include_str!("assets/icon/arc.svg");
    pub const LINE: &str = include_str!("assets/icon/line.svg");
    pub const SQUARE: &str = include_str!("assets/icon/square.svg");
    pub const TRIANGLE: &str = include_str!("assets/icon/triangle.svg");
}

pub static CURSOR_POS: std::sync::OnceLock<std::sync::Arc<std::sync::Mutex<(f32, f32)>>>
= std::sync::OnceLock::new();

type KeyHandlerMap = std::collections::HashMap<String, std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(InputEvent) + Send>>>>;
pub static KEY_HANDLERS: std::sync::OnceLock<std::sync::Arc<std::sync::Mutex<KeyHandlerMap>>> = std::sync::OnceLock::new();

pub fn key_handlers() -> std::sync::Arc<std::sync::Mutex<KeyHandlerMap>> {
    KEY_HANDLERS.get_or_init(|| std::sync::Arc::new(std::sync::Mutex::new(std::collections::HashMap::new()))).clone()
}

pub fn cursor_pos() -> (f32, f32) {
    CURSOR_POS.get()
        .map(|arc| *arc.lock().unwrap())
        .unwrap_or((0.0, 0.0))
}