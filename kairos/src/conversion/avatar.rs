use crate::conversion::KairosEvent;
use mantle::palette::Palette;
use mantle::types::theme::ThemeSet;
use petra::composites::avatar::Avatar;

pub fn convert_avatar(a: Avatar, theme: &ThemeSet, palette: &Palette) -> iced::Element<'static, KairosEvent> {
    let px = a.size.to_px(&theme.sizes);

    let bg = if let Some(ref color_source) = a.color {
        let (r, g, b, alpha) = match color_source {
            mantle::color::ColorSource::Custom(c) => c.to_rgba(),
            mantle::color::ColorSource::Palette(pc) => {
                let c = super::style::resolve_fill(&mantle::palette::Fill::Palette(*pc), palette);
                (c.r, c.g, c.b, c.a)
            }
        };
        iced::Color { r, g, b, a: alpha }
    } else {
        let palette_colors = [
            &palette.accent, &palette.primary, &palette.secondary,
            &palette.success, &palette.warning, &palette.error,
        ];
        let hash = a.name.bytes().fold(0usize, |acc, b| acc + b as usize);
        let base_color = palette_colors[hash % palette_colors.len()];
        let shift = (hash % 10) as f32 * 3.6;
        let icon_color = base_color.with_hue_shift(shift).with_alpha(0.8);
        let (r, g, b, alpha) = icon_color.to_rgba();
        iced::Color { r, g, b, a: alpha }
    };

    let letter: String = a.name.split_whitespace()
        .take(2)
        .map(|w| w.chars().next().unwrap_or('?').to_uppercase().next().unwrap())
        .collect();

    fn is_light(r: f32, g: f32, b: f32) -> bool {
        0.299 * r + 0.587 * g + 0.114 * b > 0.5
    }

    let text_color = if is_light(bg.r, bg.g, bg.b) {
        iced::Color::BLACK
    } else {
        iced::Color::WHITE
    };

    iced::widget::container(
        iced::widget::text(letter)
            .size(px * 0.6)
            .color(text_color)
    )
        .width(iced::Length::Fixed(px))
        .height(iced::Length::Fixed(px))
        .style(move |_| iced::widget::container::Style {
            background: Some(iced::Background::Color(bg)),
            border: iced::Border { radius: (px / 2.0).into(), ..Default::default() },
            ..Default::default()
        })
        .align_x(iced::alignment::Horizontal::Center)
        .align_y(iced::alignment::Vertical::Center)
        .into()
}