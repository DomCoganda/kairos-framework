use iced::widget::{container, column};
use iced::Background;
use mantle::types::theme::{ThemeSet, ThemeVariant};
use petra::primitives::column::Column;
use super::style::{resolve_fill, to_iced_length};

pub(crate) fn convert_column(c: Column, theme: &ThemeSet, variant: ThemeVariant,time:f32) -> iced::Element<'static, super::KairosEvent> {
    let palette = match variant {
        ThemeVariant::Dark => &theme.dark.palette,
        ThemeVariant::Light => &theme.light.palette,
    };
    let children: Vec<iced::Element<'static, super::KairosEvent>> = c.children
        .into_iter()
        .map(|child| super::to_iced(child, theme, variant, time))
        .collect();

    let width = to_iced_length(c.width,&theme.sizes);
    let height = to_iced_length(c.height,&theme.sizes);
    let col_height = if c.scrollable { iced::Length::Shrink } else { height };

    let iced_column = column(children)
        .width(width)
        .height(col_height)
        .spacing(match to_iced_length(c.gap,&theme.sizes) {
            iced::Length::Fixed(px) => px,
            _ => 0.0,
        })
        .padding(iced::Padding {
            top: match to_iced_length(c.padding.top,&theme.sizes) {
                iced::Length::Fixed(px) => px,
                _ => 0.0,
            },
            bottom: match to_iced_length(c.padding.bottom,&theme.sizes) {
                iced::Length::Fixed(px) => px,
                _ => 0.0,
            },
            left: match to_iced_length(c.padding.left,&theme.sizes) {
                iced::Length::Fixed(px) => px,
                _ => 0.0,
            },
            right: match to_iced_length(c.padding.right,&theme.sizes) {
                iced::Length::Fixed(px) => px,
                _ => 0.0,
            },
        })
        .align_x(
            match c.alignment {
                petra::HorizontalAlignment::Left => iced::Alignment::Start,
                petra::HorizontalAlignment::Center => iced::Alignment::Center,
                petra::HorizontalAlignment::Right => iced::Alignment::End,
                petra::HorizontalAlignment::Stretch => iced::Alignment::Center,
            }
        );

    let element: iced::Element<'static, super::KairosEvent> = if let Some(bg) = c.background {
        let color = resolve_fill(&bg, palette);
        container(iced_column)
            .width(width)
            .height(col_height)  // use col_height not height
            .style(move |_| container::Style {
                background: Some(Background::Color(color)),
                ..Default::default()
            })
            .into()
    } else {
        iced_column.into()
    };

    if c.scrollable {
        let thumb_color = if c.scrollbar.visible {
            super::style::resolve_color_source(&c.scrollbar.thumb_color, palette)
        } else {
            iced::Color::TRANSPARENT
        };
        let track_color = if c.scrollbar.visible {
            Some(Background::Color(
                super::style::resolve_color_source(&c.scrollbar.track_color, palette)
            ))
        } else {
            None
        };
        let radius = iced::border::Radius::from(c.scrollbar.thumb_radius);

        iced::widget::scrollable(element)
            .style(move |_, _| iced::widget::scrollable::Style {
                container: container::Style::default(),
                vertical_rail: iced::widget::scrollable::Rail {
                    background: track_color,
                    border: iced::Border::default(),
                    scroller: iced::widget::scrollable::Scroller {
                        color: thumb_color,
                        border: iced::border::Border {
                            radius,
                            ..Default::default()
                        },
                    },
                },
                horizontal_rail: iced::widget::scrollable::Rail {
                    background: track_color,
                    border: iced::Border::default(),
                    scroller: iced::widget::scrollable::Scroller {
                        color: thumb_color,
                        border: iced::border::Border {
                            radius,
                            ..Default::default()
                        },
                    },
                },
                gap: None,
            })
            .into()
    } else {
        element
    }
}