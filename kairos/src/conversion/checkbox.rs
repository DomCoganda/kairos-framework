use mantle::types::theme::ThemeSet;
use mantle::palette::Palette;
use petra::primitives::checkbox::Checkbox;
use crate::conversion::KairosEvent;
use crate::conversion::style::{resolve_color_source, resolve_fill};

pub(crate) fn convert_checkbox(c: Checkbox, palette: &Palette, _theme: &ThemeSet) -> iced::Element<'static, KairosEvent> {
    let is_checked = c.state.get();
    let on_change = c.on_change.clone();
    let sig = c.state.clone();
    let label = c.label.map(|t| t.content).unwrap_or_default();

    let checked_bg = resolve_fill(&c.style.checked.background, palette);
    let unchecked_bg = resolve_fill(&c.style.unchecked.background, palette);
    let checked_mark = resolve_color_source(&c.style.checked.checkmark, palette);

    iced::widget::checkbox(label, is_checked)
        .on_toggle(move |val| {
            sig.set(val);
            if let Some(ref cb) = on_change {
                if let Ok(mut f) = cb.lock() { f(val); }
            }
            KairosEvent::None
        })
        .style(move |_, status| {
            let is_checked = matches!(status,
                iced::widget::checkbox::Status::Active { is_checked: true } |
                iced::widget::checkbox::Status::Hovered { is_checked: true }
            );
            iced::widget::checkbox::Style {
                background: iced::Background::Color(if is_checked { checked_bg } else { unchecked_bg }),
                icon_color: checked_mark,
                border: iced::Border {
                    radius: iced::border::Radius::from(4.0),
                    width: 0.0,
                    color: iced::Color::TRANSPARENT,
                },
                text_color: None,
            }
        })
        .into()
}