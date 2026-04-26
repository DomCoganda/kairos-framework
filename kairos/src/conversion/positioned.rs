use iced::widget::container;
use mantle::types::theme::{ThemeSet, ThemeVariant};
use petra::Positioned;
use crate::conversion::style::to_iced_length;

pub(crate) fn convert_positioned(p: Positioned, theme: &ThemeSet, variant: ThemeVariant, time: f32) -> iced::Element<'static, super::KairosEvent> {
    let child = super::to_iced(*p.child, theme, variant, time);
    let width = to_iced_length(p.width, &theme.sizes);
    let height = to_iced_length(p.height, &theme.sizes);

    container(child)
        .padding(iced::Padding {
            top: p.y,
            left: p.x,
            right: 0.0,
            bottom: 0.0,
        })
        .width(width)
        .height(height)
        .into()
}