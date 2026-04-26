use iced::widget::{container, row};
use iced_aw::wrap::Wrap;
use iced::{Background};
use mantle::types::theme::{ThemeSet, ThemeVariant};
use petra::primitives::row::Row;
use super::style::{resolve_fill, to_iced_length};

pub(crate) fn convert_row(r: Row, theme: &ThemeSet, variant: ThemeVariant, time:f32) -> iced::Element<'static, super::KairosEvent> {
    let palette = match variant {
        ThemeVariant::Dark => &theme.dark.palette,
        ThemeVariant::Light => &theme.light.palette,
    };
    let children: Vec<iced::Element<'static, super::KairosEvent>> = r.children
        .into_iter()
        .map(|child| super::to_iced(child, theme, variant, time))
        .collect();

    let width = to_iced_length(r.width,&theme.sizes);
    let height = to_iced_length(r.height,&theme.sizes);

    if r.wrap {
        let wrap = Wrap::with_elements(children)
            .max_width(match width {
                iced::Length::Fixed(px) => px,
                _ => f32::INFINITY,
            })
            .spacing(match to_iced_length(r.gap,&theme.sizes) {
                iced::Length::Fixed(px) => px,
                _ => 0.0,
            })
            .padding(match to_iced_length(r.padding.left,&theme.sizes) {
                iced::Length::Fixed(px) => px,
                _ => 0.0,
            });

        if let Some(bg) = r.background {
            let color = resolve_fill(&bg, palette);
            container(wrap)
                .width(width)
                .height(height)
                .style(move |_| container::Style {
                    background: Some(Background::Color(color)),
                    ..Default::default()
                })
                .into()
        } else {
            wrap.into()
        }
    } else {
        let iced_row = row(children)
            .width(width)
            .height(height)
            .spacing(match to_iced_length(r.gap,&theme.sizes) {
                iced::Length::Fixed(px) => px,
                _ => 0.0,
            })
            .padding(iced::Padding {
                top: match to_iced_length(r.padding.top,&theme.sizes) {
                    iced::Length::Fixed(px) => px,
                    _ => 0.0,
                },
                bottom: match to_iced_length(r.padding.bottom,&theme.sizes) {
                    iced::Length::Fixed(px) => px,
                    _ => 0.0,
                },
                left: match to_iced_length(r.padding.left,&theme.sizes) {
                    iced::Length::Fixed(px) => px,
                    _ => 0.0,
                },
                right: match to_iced_length(r.padding.right,&theme.sizes) {
                    iced::Length::Fixed(px) => px,
                    _ => 0.0,
                },
            })
        .align_y(
            match r.alignment {
                petra::VerticalAlignment::Top => iced::Alignment::Start,
                petra::VerticalAlignment::Center => iced::Alignment::Center,
                petra::VerticalAlignment::Bottom => iced::Alignment::End,
                petra::VerticalAlignment::Stretch => iced::Alignment::Center,
            }
        );

        if let Some(bg) = r.background {
            let color = resolve_fill(&bg, palette);
            container(iced_row)
                .width(width)
                .height(height)
                .style(move |_| container::Style {
                    background: Some(Background::Color(color)),
                    ..Default::default()
                })
                .into()
        } else {
            iced_row.into()
        }
    }
}