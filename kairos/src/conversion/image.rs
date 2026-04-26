use mantle::palette::Palette;
use mantle::types::theme::ThemeSet;
use mantle::source::FileSource;
use petra::primitives::image::{Image, ImageFit};
use crate::conversion::KairosEvent;
use crate::conversion::style::to_iced_length;

pub(crate) fn convert_image(
    img: Image,
    _palette: &Palette,
    theme: &ThemeSet,
) -> iced::Element<'static, KairosEvent> {
    let width = to_iced_length(img.width, &theme.sizes);
    let height = to_iced_length(img.height, &theme.sizes);

    let content_fit = match img.fit {
        ImageFit::Contain => iced::ContentFit::Contain,
        ImageFit::Cover   => iced::ContentFit::Cover,
        ImageFit::Fill    => iced::ContentFit::Fill,
        ImageFit::None    => iced::ContentFit::None,
    };

    let handle = match img.source {
        FileSource::File(path) if !path.is_empty() && std::path::Path::new(&path).exists() => {
            Some(iced::widget::image::Handle::from_path(path))
        }
        _ => None,
    };

    match handle {
        Some(h) => iced::widget::image(h)
            .width(width)
            .height(height)
            .content_fit(content_fit)
            .into(),
        None => iced::widget::container(
            iced::widget::Space::new(width, height)
        )
            .width(width)
            .height(height)
            .style(|_| iced::widget::container::Style {
                background: Some(iced::Background::Color(iced::Color::from_rgb(
                    0x0A as f32 / 255.0,
                    0x0A as f32 / 255.0,
                    0x2A as f32 / 255.0,
                ))),
                ..Default::default()
            })
            .into(),
    }
}