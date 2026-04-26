use iced::Background;
use iced::widget::container;
use mantle::palette::Palette;
use mantle::types::theme::ThemeSet;
use petra::primitives::spacer::Spacer;
use petra::primitives::types::Orientation;
use petra::Space;
use super::style::{to_iced_length, to_iced_color};

pub(crate) fn convert_spacer(s: Spacer, theme: &ThemeSet, _palette: &Palette) -> iced::Element<'static, super::KairosEvent> {
    let (width, height) = match s.orientation {
        Orientation::Vertical => (to_iced_length(Space::Shrink, &theme.sizes), to_iced_length(s.size, &theme.sizes)),
        Orientation::Horizontal => (to_iced_length(s.size, &theme.sizes), to_iced_length(Space::Shrink, &theme.sizes)),
    };

    if let Some(bg) = s.background {
        let color = to_iced_color(bg);
        container(iced::widget::Space::new(width, height))
            .width(width)
            .height(height)
            .style(move |_| container::Style {
                background: Some(Background::Color(color)),
                ..Default::default()
            })
            .into()
    } else {
        iced::widget::Space::new(width, height).into()
    }
}