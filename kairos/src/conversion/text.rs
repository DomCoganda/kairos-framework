use iced::widget::text;
use mantle::types::theme::{ThemeSet};
use mantle::typography::TextStyle;
use mantle::palette::Palette;
use crate::conversion::style::to_iced_length;

pub(crate) fn convert_text(t: petra::Text, theme: &ThemeSet, palette: &Palette) -> iced::Element<'static, super::KairosEvent> {
    let font_def = match t.style {
        TextStyle::H1 => &theme.typography.h1,
        TextStyle::H2 => &theme.typography.h2,
        TextStyle::H3 => &theme.typography.h3,
        TextStyle::H4 => &theme.typography.h4,
        TextStyle::Body => &theme.typography.body,
        TextStyle::Caption => &theme.typography.caption,
        TextStyle::Label => &theme.typography.label,
        TextStyle::Code => &theme.typography.code,
    };

    let weight = match font_def.weight {
        mantle::typography::FontWeight::Thin => iced::font::Weight::Thin,
        mantle::typography::FontWeight::ExtraLight => iced::font::Weight::ExtraLight,
        mantle::typography::FontWeight::Light => iced::font::Weight::Light,
        mantle::typography::FontWeight::Regular => iced::font::Weight::Normal,
        mantle::typography::FontWeight::Medium => iced::font::Weight::Medium,
        mantle::typography::FontWeight::Semibold => iced::font::Weight::Semibold,
        mantle::typography::FontWeight::Bold => iced::font::Weight::Bold,
        mantle::typography::FontWeight::ExtraBold => iced::font::Weight::ExtraBold,
        mantle::typography::FontWeight::Black => iced::font::Weight::Black,
    };

    let style = match font_def.style {
        mantle::typography::FontStyle::Italic => iced::font::Style::Italic,
        mantle::typography::FontStyle::Normal => iced::font::Style::Normal,
        mantle::typography::FontStyle::Oblique => iced::font::Style::Oblique,
    };

    let content = if t.content.is_empty() { " ".to_string() } else { t.content };
    let mut widget = text(content)
        .size(font_def.size)
        .font(iced::Font { weight, style, ..iced::Font::default() });

    if let Some(ref color) = t.color {
        let resolved = super::style::resolve_color_source(color, palette);
        widget = widget.color(resolved);
    }

    if let Some(alignment) = t.alignment {
        widget = widget.align_x(match alignment {
            petra::HorizontalAlignment::Left => iced::alignment::Horizontal::Left,
            petra::HorizontalAlignment::Center => iced::alignment::Horizontal::Center,
            petra::HorizontalAlignment::Right => iced::alignment::Horizontal::Right,
            petra::HorizontalAlignment::Stretch => iced::alignment::Horizontal::Left,
        });
    }
    if let Some(w) = t.width {
        widget = widget.width(to_iced_length(w, &theme.sizes));
    }

    widget.into()
}