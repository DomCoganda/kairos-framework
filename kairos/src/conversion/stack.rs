use iced::Background;
use iced::widget::container;
use mantle::types::theme::{ThemeSet, ThemeVariant};
use petra::Stack;
use crate::conversion::style::{resolve_fill, to_iced_length};
use iced::widget::stack;
use super::positioned;

pub(crate) fn convert_stack(s: Stack, theme: &ThemeSet, variant: ThemeVariant, time: f32) -> iced::Element<'static, super::KairosEvent> {
    let palette = match variant {
        ThemeVariant::Dark => &theme.dark.palette,
        ThemeVariant::Light => &theme.light.palette,
    };
    let children: Vec<iced::Element<'static, super::KairosEvent>> = s.children
        .into_iter()
        .rev()
        .map(|child| super::to_iced(child, theme, variant, time))
        .chain(
            s.positioned
                .into_iter()
                .map(|mut p| {
                    p.x -= s.offset.0;
                    p.y -= s.offset.1;
                    positioned::convert_positioned(p, theme, variant, time)
                })
        )
        .collect();

    let width = to_iced_length(s.width, &theme.sizes);
    let height = to_iced_length(s.height, &theme.sizes);
    let padding = iced::Padding {
        top: match to_iced_length(s.padding.top, &theme.sizes) { iced::Length::Fixed(px) => px, _ => 0.0 },
        bottom: match to_iced_length(s.padding.bottom, &theme.sizes) { iced::Length::Fixed(px) => px, _ => 0.0 },
        left: match to_iced_length(s.padding.left, &theme.sizes) { iced::Length::Fixed(px) => px, _ => 0.0 },
        right: match to_iced_length(s.padding.right, &theme.sizes) { iced::Length::Fixed(px) => px, _ => 0.0 },
    };

    let iced_stack = stack(children)
        .width(width)
        .height(height);

    if let Some(bg) = s.background {
        let color = resolve_fill(&bg, palette);
        container(iced_stack)
            .width(width)
            .height(height)
            .padding(padding)
            .style(move |_| container::Style {
                background: Some(Background::Color(color)),
                ..Default::default()
            })
            .into()
    } else {
        container(iced_stack)
            .width(width)
            .height(height)
            .padding(padding)
            .into()
    }
}