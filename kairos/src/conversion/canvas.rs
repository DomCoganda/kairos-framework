use iced::widget::container;
use iced::Background;
use petra::primitives::canvas::Canvas;
use super::style::{resolve_fill, to_iced_length};
use mantle::palette::Palette;
use mantle::types::theme::ThemeSet;

pub(crate) fn convert_canvas(c: Canvas, palette: &Palette, theme: &ThemeSet) -> iced::Element<'static, super::KairosEvent> {
    let width = to_iced_length(c.width, &theme.sizes);
    let height = to_iced_length(c.height,&theme.sizes);

    let mut cont = container(iced::widget::Space::new(width, height))
        .width(width)
        .height(height);

    if let Some(bg) = c.background {
        let color = resolve_fill(&bg, palette);
        cont = cont.style(move |_| container::Style {
            background: Some(Background::Color(color)),
            ..Default::default()
        });
    }

    cont.into()
}