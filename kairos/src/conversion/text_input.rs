use iced::widget::{column, container, row, text, text_input};
use iced::Background;
use mantle::palette::Palette;
use mantle::types::theme::{ThemeSet, ThemeVariant};
use crate::conversion::style::{resolve_color_source, resolve_fill, to_iced_length};
use crate::conversion::KairosEvent;

pub(crate) fn convert_text_input(
    input: petra::TextInput,
    palette: &Palette,
    theme: &ThemeSet,
    variant: ThemeVariant,
) -> iced::Element<'static, KairosEvent> {
    let placeholder = input.placeholder.map(|t| t.content).unwrap_or_default();
    let current_value = input.value.get_clone();
    let signal = input.value.clone();
    let bg_color = resolve_fill(&input.style.background, palette);
    let text_color = resolve_color_source(&input.style.text_color, palette);
    let width = to_iced_length(input.width, &theme.sizes);
    let height = to_iced_length(input.height, &theme.sizes);
    let radius: iced::border::Radius = match input.style.border.radius {
        mantle::border::BorderRadius::Sharp => 0.0.into(),
        mantle::border::BorderRadius::Rounded(r) => r.into(),
        mantle::border::BorderRadius::Pill => 999.0.into(),
    };
    let thickness = input.style.border.thickness;
    let border_color = resolve_color_source(&input.style.border.color, palette);

    let input_widget: iced::Element<'static, KairosEvent> = text_input(&placeholder, &current_value)
        .on_input(move |s| KairosEvent::StringInput(signal.clone(), s))
        .width(width)
        .style(move |theme, status| {
            let mut s = iced::widget::text_input::default(theme, status);
            s.border.radius = radius;
            s.border.width = thickness;
            s.border.color = border_color;
            s.background = Background::Color(bg_color);
            s.value = text_color;
            s
        })
        .into();

    let with_ends: iced::Element<'static, KairosEvent> = match (input.prefix, input.suffix) {
        (None, None) => input_widget,
        (prefix, suffix) => {
            let mut items: Vec<iced::Element<'static, KairosEvent>> = vec![];
            if let Some(p) = prefix {
                items.push(match p {
                    petra::InputEnds::Text(t) => text(t.content).into(),
                    petra::InputEnds::Icon(w) => super::to_iced(*w, theme, variant, 0.0),
                });
            }
            items.push(input_widget);
            if let Some(s) = suffix {
                items.push(match s {
                    petra::InputEnds::Text(t) => text(t.content).into(),
                    petra::InputEnds::Icon(w) => super::to_iced(*w, theme, variant, 0.0),
                });
            }
            row(items)
                .align_y(iced::Alignment::Center)
                .into()
        }
    };

    let mut outer: Vec<iced::Element<'static, KairosEvent>> = vec![];
    if let Some(label) = input.label {
        outer.push(text(label.content).into());
    }
    outer.push(with_ends);
    if let Some(error) = input.error {
        let error_color = resolve_color_source(
            &mantle::color::ColorSource::Palette(mantle::palette::PaletteColor::Error),
            palette,
        );
        outer.push(text(error.content).color(error_color).into());
    }

    if outer.len() == 1 {
        container(outer.remove(0)).width(width).height(height).into()
    } else {
        column(outer).width(width).spacing(4.0).into()
    }
}