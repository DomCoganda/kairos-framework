use iced::widget::svg;
use petra::Icon;
use crate::conversion::KairosEvent;

pub fn icon_to_iced(icon: Icon, theme: &mantle::types::theme::ThemeSet, palette: &mantle::palette::Palette) -> iced::Element<'static, KairosEvent> {
    let rotation = iced::Radians(icon.rotation * std::f32::consts::PI / 180.0);
    let px = icon.size.to_px(&theme.sizes);

    let handle = match &icon.source {
        mantle::icons::IconSource::Raw(s) => svg::Handle::from_memory(s.as_bytes().to_vec()),
        mantle::icons::IconSource::Path(p) => match p {
            mantle::source::FileSource::File(s) => svg::Handle::from_path(s),
            mantle::source::FileSource::Embedded(s) => svg::Handle::from_memory(s.as_bytes().to_vec()),
            mantle::source::FileSource::Url(s) => svg::Handle::from_path(s),
        },
        mantle::icons::IconSource::System(s) => svg::Handle::from_path(s),
    };

    let svg_base = iced::widget::svg(handle)
        .rotation(rotation)
        .width(iced::Length::Fixed(px))
        .height(iced::Length::Fixed(px));

    match &icon.color {
        Some(c) => {
            let color = super::style::resolve_color_source(c, palette);
            svg_base.style(move |_, _| svg::Style { color: Some(color) }).into()
        },
        None => svg_base.into(),
    }
}