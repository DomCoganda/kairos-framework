use mantle::types::theme::ThemeSet;
use mantle::palette::Palette;
use petra::primitives::toggle::Toggle;
use crate::conversion::KairosEvent;
use crate::conversion::style::{resolve_color_source, resolve_fill};

pub(crate) fn convert_toggle(t: Toggle, palette: &Palette, _theme: &ThemeSet) -> iced::Element<'static, KairosEvent> {
    let is_on = t.state.get();
    let on_change = t.on_change.clone();
    let sig = t.state.clone();

    let on_bg = resolve_fill(&t.style.on.background, palette);
    let off_bg = resolve_fill(&t.style.off.background, palette);
    let on_handle = resolve_color_source(&t.style.on.handle, palette);
    let off_handle = resolve_color_source(&t.style.off.handle, palette);

    iced::widget::toggler(is_on)
        .on_toggle(move |val| {
            sig.set(val);
            if let Some(ref cb) = on_change {
                if let Ok(mut f) = cb.lock() { f(val); }
            }
            KairosEvent::None
        })
        .style(move |_, status| {
            let active = matches!(status, iced::widget::toggler::Status::Active { is_toggled: true } | iced::widget::toggler::Status::Hovered { is_toggled: true });
            iced::widget::toggler::Style {
                background: if active { on_bg } else { off_bg },
                background_border_width: 0.0,
                background_border_color: iced::Color::TRANSPARENT,
                foreground: if active { on_handle } else { off_handle },
                foreground_border_width: 0.0,
                foreground_border_color: iced::Color::TRANSPARENT,
            }
        })
        .into()
}