pub mod composites;
pub mod primitives;
pub mod helpers;

pub use helpers::color::hex;
pub use helpers::clickable::clickable;
pub use helpers::scrollable::scrollable;

pub use primitives::widget::Widget;
pub use primitives::window::Window;
pub use primitives::button::Button;
pub use primitives::row::Row;
pub use primitives::column::Column;
pub use primitives::canvas::Canvas;
pub use primitives::text::Text;
pub use primitives::types::Space;
pub use primitives::types::LabelContent;
pub use primitives::types::HorizontalAlignment;
pub use primitives::types::VerticalAlignment;
pub use primitives::padding::Padding;
pub use primitives::padding::AxisPadding;
pub use primitives::spacer::Spacer;
pub use primitives::graph::Graph;
pub use primitives::divider::Divider;
pub use primitives::text_input::TextInput;
pub use primitives::text_input::InputEnds;
pub use primitives::input_event::Binding;
pub use primitives::input_event::MouseButton;
pub use primitives::input_event::Key;
pub use primitives::input_event::InputEvent;
pub use primitives::stack::Stack;
pub use primitives::positioned::Positioned;
pub use primitives::spinner::Spinner;
pub use primitives::icon::Icon;
pub use primitives::types::Orientation;
pub use primitives::scrubber::Scrubber;
pub use primitives::toggle::Toggle;
pub use primitives::checkbox::Checkbox;
pub use primitives::slider::Slider;
pub use primitives::radio::Radio;
pub use primitives::image::Image;
pub use primitives::image::ImageFit;


pub use helpers::styles::WidgetStyle;

pub use composites::avatar::Avatar;
pub use composites::switcher::Switcher;
pub use composites::dropdown::Dropdown;
pub use composites::progress_bar::ProgressBar;


pub use nuntius::signal::Signal;
pub use nuntius::buffer::SignalBuffer;

pub use mantle::sizing::Size;
pub use mantle::border::BorderRadius;
pub use mantle::line::LineStyle;
pub use mantle::icons::IconStyle;
pub use mantle::palette::PaletteColor;
pub use mantle::palette::Fill;
pub use mantle::palette::Palette;
pub use mantle::icons::IconSource;
pub use mantle::shape::SpinnerMotion;
pub use mantle::shape::Direction;

pub trait Component {
    fn build(&self, palette: &Palette) -> Widget;

    fn deferred(self) -> Widget
    where Self: Sized + Send + Sync + Clone + 'static
    {
        let this = self.clone();
        Widget::Deferred(std::sync::Arc::new(move |p| {
            let this = this.clone();
            this.build(p)
        }))
    }
}
pub trait Buildable {
    fn resolve(self, palette: &Palette) -> Widget;
}
impl Buildable for Widget {
    fn resolve(self, _palette: &Palette) -> Widget {
        self
    }
}

impl<C: Component + 'static> Buildable for C {
    fn resolve(self, palette: &Palette) -> Widget {
        self.build(palette)
    }
}
