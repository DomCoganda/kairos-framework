use mantle::types::theme::ThemeSet;
use mantle::palette::Palette;
use petra::ProgressBar;
use crate::conversion::KairosEvent;
use crate::conversion::style::{resolve_color_source, to_iced_length};

pub(crate) fn convert_progress_bar(p: ProgressBar, palette: &Palette, theme: &ThemeSet) -> iced::Element<'static, KairosEvent> {
    let fill = resolve_color_source(&p.color, palette);
    let bg = resolve_color_source(&p.background, palette);
    let width = to_iced_length(p.width, &theme.sizes);

    iced::widget::progress_bar(p.min..=p.max, p.value)
        .width(width)
        .height(p.height)
        .style(move |_| iced::widget::progress_bar::Style {
            background: iced::Background::Color(bg),
            bar: iced::Background::Color(fill),
            border: iced::Border::default(),
        })
        .into()
}