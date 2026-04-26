use mantle::types::theme::ThemeSet;
use mantle::palette::Palette;
use petra::primitives::radio::Radio;
use crate::conversion::KairosEvent;
use crate::conversion::style::{resolve_color_source, resolve_fill};

pub(crate) fn convert_radio(r: Radio, palette: &Palette, _theme: &ThemeSet) -> iced::Element<'static, KairosEvent> {
    let selected = r.selected.get_clone();
    let is_selected = selected == r.value;
    let on_select = r.on_select.clone();
    let value = r.value.clone();
    let label = r.label.map(|t| t.content).unwrap_or_default();

    let selected_bg = resolve_fill(&r.style.selected, palette);
    let unselected_bg = resolve_fill(&r.style.unselected, palette);
    let dot_color = resolve_color_source(&r.style.dot, palette);

    // Use bool as the copy-able value type
    iced::widget::radio(label, true, Some(is_selected), move |_| {
        if let Some(ref cb) = on_select {
            if let Ok(mut f) = cb.lock() { f(value.clone()); }
        }
        KairosEvent::None
    })
        .style(move |_, status| {
            let active = matches!(status,
                iced::widget::radio::Status::Active { is_selected: true } |
                iced::widget::radio::Status::Hovered { is_selected: true }
            );
            iced::widget::radio::Style {
                background: iced::Background::Color(if active { selected_bg } else { unselected_bg }),
                dot_color,
                border_width: 0.0,
                border_color: iced::Color::TRANSPARENT,
                text_color: None,
            }
        })
        .into()
}