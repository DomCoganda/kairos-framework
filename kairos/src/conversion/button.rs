use iced::widget::{button, row, text};
use mantle::palette::Palette;
use petra::primitives::button::Button;
use petra::primitives::types::LabelContent;
use crate::conversion::KairosEvent;
use super::style::{to_iced_length, resolve_color_source};

pub(crate) fn convert_button(b: Button, palette: &Palette, theme: &mantle::types::theme::ThemeSet, variant: mantle::types::theme::ThemeVariant, time: f32) -> iced::Element<'static, super::KairosEvent> {
    let raw_label: iced::Element<'static, super::KairosEvent> = match b.label {
        LabelContent::Label(t) => {
            let content = if t.content.is_empty() { " ".to_string() } else { t.content };
            let mut w = text(content).size(match t.style {
                mantle::typography::TextStyle::H1 => theme.typography.h1.size,
                mantle::typography::TextStyle::H2 => theme.typography.h2.size,
                mantle::typography::TextStyle::H3 => theme.typography.h3.size,
                mantle::typography::TextStyle::H4 => theme.typography.h4.size,
                mantle::typography::TextStyle::Body => theme.typography.body.size,
                mantle::typography::TextStyle::Caption => theme.typography.caption.size,
                mantle::typography::TextStyle::Label => theme.typography.label.size,
                mantle::typography::TextStyle::Code => theme.typography.code.size,
            });
            if let Some(color) = t.color {
                w = w.color(resolve_color_source(&color, palette));
            }
            if let Some(alignment) = t.alignment {
                w = w.align_x(match alignment {
                    petra::HorizontalAlignment::Left => iced::alignment::Horizontal::Left,
                    petra::HorizontalAlignment::Center => iced::alignment::Horizontal::Center,
                    petra::HorizontalAlignment::Right => iced::alignment::Horizontal::Right,
                    petra::HorizontalAlignment::Stretch => iced::alignment::Horizontal::Left,
                });
            }
            w.into()
        },
        LabelContent::Icon(w) => {
            let icon_el = super::to_iced(*w, theme, variant, time);
            let is_padded = matches!(
                to_iced_length(b.padding.left, &theme.sizes),
                iced::Length::Fixed(px) if px > 0.0
            );
            if is_padded {
                icon_el
            } else {
                iced::widget::container(icon_el)
                    .width(iced::Length::Fill)
                    .height(iced::Length::Fill)
                    .align_x(iced::alignment::Horizontal::Center)
                    .align_y(iced::alignment::Vertical::Center)
                    .into()
            }
        },
        LabelContent::Both(t, w) => {
            let icon_el = super::to_iced(*w, theme, variant, time);
            let text_el = text(if t.content.is_empty() { " ".to_string() } else { t.content })
                .size(match t.style {
                    mantle::typography::TextStyle::H1 => theme.typography.h1.size,
                    mantle::typography::TextStyle::H2 => theme.typography.h2.size,
                    mantle::typography::TextStyle::H3 => theme.typography.h3.size,
                    mantle::typography::TextStyle::H4 => theme.typography.h4.size,
                    mantle::typography::TextStyle::Body => theme.typography.body.size,
                    mantle::typography::TextStyle::Caption => theme.typography.caption.size,
                    mantle::typography::TextStyle::Label => theme.typography.label.size,
                    mantle::typography::TextStyle::Code => theme.typography.code.size,
                });
            let text_color = if let Some(color) = t.color {
                resolve_color_source(&color, palette)
            } else {
                resolve_color_source(
                    &mantle::color::ColorSource::Palette(mantle::palette::PaletteColor::Text),
                    palette,
                )
            };
            iced::widget::container(
                row(vec![
                    icon_el,
                    text_el.color(text_color).into(),
                ])
                    .spacing(8.0)
                    .align_y(iced::Alignment::Center)
            )
                .width(iced::Length::Fill)
                .height(iced::Length::Fill)
                .align_x(iced::alignment::Horizontal::Center)
                .align_y(iced::alignment::Vertical::Center)
                .into()
        },
    };

    let label: iced::Element<'static, super::KairosEvent> = iced::widget::container(raw_label)
        .width(iced::Length::Fill)
        .height(iced::Length::Fill)
        .align_x(iced::alignment::Horizontal::Center)
        .align_y(iced::alignment::Vertical::Center)
        .into();

    let bg_color = super::style::resolve_fill(&b.style.background, palette);

    let text_color = match &b.style.text_color {
        mantle::color::ColorSource::Palette(pc) => super::style::resolve_fill(
            &mantle::palette::Fill::Palette(*pc), palette
        ),
        mantle::color::ColorSource::Custom(c) => {
            let (r, g, b, a) = c.to_rgba();
            iced::Color { r, g, b, a }
        }
    };

    let width = to_iced_length(b.width, &theme.sizes);
    let height = to_iced_length(b.height, &theme.sizes);

    let radius = match b.style.border.radius {
        mantle::border::BorderRadius::Sharp => iced::border::Radius::from(0.0),
        mantle::border::BorderRadius::Rounded(r) => iced::border::Radius::from(r),
        mantle::border::BorderRadius::Pill => iced::border::Radius::from(999.0),
    };

    let mut iced_button = button(label)
        .width(width)
        .height(height)
        .padding(iced::Padding {
            top: match to_iced_length(b.padding.top, &theme.sizes) { iced::Length::Fixed(px) => px, _ => 0.0 },
            bottom: match to_iced_length(b.padding.bottom, &theme.sizes) { iced::Length::Fixed(px) => px, _ => 0.0 },
            left: match to_iced_length(b.padding.left, &theme.sizes) { iced::Length::Fixed(px) => px, _ => 0.0 },
            right: match to_iced_length(b.padding.right, &theme.sizes) { iced::Length::Fixed(px) => px, _ => 0.0 },
        });

    let shadow = b.style.shadow.clone();

    if let Some(on_press) = b.on_press {
        iced_button = iced_button.on_press(KairosEvent::Press(on_press));
    }

    iced_button = iced_button.style(move |_, _| iced::widget::button::Style {
        background: Some(iced::Background::Color(bg_color)),
        text_color,
        border: iced::Border { radius, ..Default::default() },
        shadow: shadow.as_ref().map(|s| super::style::to_iced_shadow(s)).unwrap_or_default(),
        ..Default::default()
    });

    let button_el: iced::Element<'static, KairosEvent> = iced_button.into();

    let border = b.style.border;
    if border.thickness == 0.0 {
        return button_el;
    }

    let color = super::style::to_iced_border(&border, palette).color;
    let thickness = border.thickness;
    let (show_top, show_bottom, show_left, show_right) = match &border.sides {
        mantle::border::BorderSides::All => (true, true, true, true),
        mantle::border::BorderSides::None => (false, false, false, false),
        mantle::border::BorderSides::Custom { top, bottom, left, right } => (*top, *bottom, *left, *right),
        mantle::border::BorderSides::Horizontal => (false, false, true, true),
        mantle::border::BorderSides::Vertical => (true, true, false, false),
    };

    let right_bar: iced::Element<'static, KairosEvent> = iced::widget::container(
        iced::widget::Space::new(iced::Length::Fixed(thickness), iced::Length::Fill)
    ).style(move |_| iced::widget::container::Style {
        background: if show_right { Some(iced::Background::Color(color)) } else { None },
        ..Default::default()
    }).into();

    let bottom_bar: iced::Element<'static, KairosEvent> = iced::widget::container(
        iced::widget::Space::new(iced::Length::Fill, iced::Length::Fixed(thickness))
    ).style(move |_| iced::widget::container::Style {
        background: if show_bottom { Some(iced::Background::Color(color)) } else { None },
        ..Default::default()
    }).into();

    let top_bar: iced::Element<'static, KairosEvent> = iced::widget::container(
        iced::widget::Space::new(iced::Length::Fill, iced::Length::Fixed(thickness))
    ).style(move |_| iced::widget::container::Style {
        background: if show_top { Some(iced::Background::Color(color)) } else { None },
        ..Default::default()
    }).into();

    let left_bar: iced::Element<'static, KairosEvent> = iced::widget::container(
        iced::widget::Space::new(iced::Length::Fixed(thickness), iced::Length::Fill)
    ).style(move |_| iced::widget::container::Style {
        background: if show_left { Some(iced::Background::Color(color)) } else { None },
        ..Default::default()
    }).into();

    iced::widget::column(vec![
        top_bar,
        iced::widget::row(vec![left_bar, button_el, right_bar]).into(),
        bottom_bar,
    ])
        .width(width)
        .height(height)
        .into()
}