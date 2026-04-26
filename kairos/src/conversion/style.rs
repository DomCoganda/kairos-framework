use mantle::color::ColorSource;
use mantle::visual::SurfaceStyle;
use petra::primitives::types::Space;
use mantle::sizing::SizeScale;

pub(crate) fn to_iced_length(space: Space, sizes: &SizeScale) -> iced::Length {
    match space {
        Space::Fill(n) => iced::Length::FillPortion(n),
        Space::Shrink => iced::Length::Shrink,
        Space::Custom(px) => iced::Length::Fixed(px),
        Space::Scale(size) => iced::Length::Fixed(size.to_px(sizes)),
    }
}

pub(crate) fn to_iced_color(surface: SurfaceStyle) -> iced::Color {
    match surface {
        SurfaceStyle::Solid(source) => match source {
            ColorSource::Custom(color) => {
                let (r, g, b, a) = color.to_rgba();
                iced::Color { r, g, b, a }
            },
            ColorSource::Palette(_) => iced::Color::TRANSPARENT,
        },
        SurfaceStyle::Effect(_) => iced::Color::TRANSPARENT,
    }
}

pub(crate) fn resolve_fill(fill: &mantle::palette::Fill, palette: &mantle::palette::Palette) -> iced::Color {
    match fill {
        mantle::palette::Fill::Surface => to_iced_color(palette.surface.clone()),
        mantle::palette::Fill::Palette(pc) => {
            let color = match pc {
                mantle::palette::PaletteColor::Primary => &palette.primary,
                mantle::palette::PaletteColor::Secondary => &palette.secondary,
                mantle::palette::PaletteColor::Accent => &palette.accent,
                mantle::palette::PaletteColor::Text => &palette.text,
                mantle::palette::PaletteColor::Background => &palette.background,
                mantle::palette::PaletteColor::Error => &palette.error,
                mantle::palette::PaletteColor::Warning => &palette.warning,
                mantle::palette::PaletteColor::Success => &palette.success,
            };
            let (r, g, b, a) = color.to_rgba();
            iced::Color { r, g, b, a }
        },
        mantle::palette::Fill::Custom(surface) => to_iced_color(surface.clone()),
    }
}

pub(crate) fn to_iced_border(border: &mantle::border::Border, palette: &mantle::palette::Palette) -> iced::Border {
    let radius = match &border.radius {
        mantle::border::BorderRadius::Sharp => iced::border::Radius::from(0.0),
        mantle::border::BorderRadius::Rounded(r) => iced::border::Radius::from(*r),
        mantle::border::BorderRadius::Pill => iced::border::Radius::from(999.0),
    };
    let color = match &border.color {
        mantle::color::ColorSource::Custom(c) => {
            let (r, g, b, a) = c.to_rgba();
            iced::Color { r, g, b, a }
        },
        mantle::color::ColorSource::Palette(pc) => {
            resolve_fill(&mantle::palette::Fill::Palette(*pc), palette)
        },
    };
    iced::Border {
        color,
        width: border.thickness,
        radius,
    }
}

pub(crate) fn resolve_color_source(source: &ColorSource, palette: &mantle::palette::Palette) -> iced::Color {
    match source {
        ColorSource::Custom(c) => {
            let (r, g, b, a) = c.to_rgba();
            iced::Color { r, g, b, a }
        },
        ColorSource::Palette(pc) => {
            resolve_fill(&mantle::palette::Fill::Palette(*pc), palette)
        },
    }
}

pub(crate) fn to_iced_shadow(shadow: &mantle::visual::Shadow) -> iced::Shadow {
    let (r, g, b, a) = shadow.color.to_rgba();
    iced::Shadow {
        color: iced::Color { r, g, b, a },
        offset: iced::Vector::new(shadow.offset_x, shadow.offset_y),
        blur_radius: shadow.blur,
    }
}

#[allow(dead_code)]
pub(crate) fn color_to_hex(color: &mantle::color::ColorSource, palette: &mantle::palette::Palette) -> String {
    let (r, g, b) = match color {
        mantle::color::ColorSource::Custom(c) => {
            let (r, g, b, _) = c.to_rgba();
            (r, g, b)
        },
        mantle::color::ColorSource::Palette(pc) => {
            let c = resolve_fill(&mantle::palette::Fill::Palette(*pc), palette);
            (c.r, c.g, c.b)
        }
    };
    format!("#{:02x}{:02x}{:02x}", (r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8)
}