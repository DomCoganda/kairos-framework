use mantle::types::theme::ThemeSet;
use mantle::palette::Palette;
use petra::primitives::slider::Slider;
use crate::conversion::KairosEvent;
use crate::conversion::style::resolve_color_source;

pub(crate) fn convert_slider(s: Slider, palette: &Palette, _theme: &ThemeSet) -> iced::Element<'static, KairosEvent> {
    let value = s.value.get();
    let on_change = s.on_change.clone();
    let sig = s.value.clone();

    let track_color = resolve_color_source(&s.style.track, palette);
    let fill_color = resolve_color_source(&s.style.fill, palette);
    let handle_color = resolve_color_source(&s.style.handle, palette);

    iced::widget::slider(s.min..=s.max, value, move |val| {
        sig.set(val);
        if let Some(ref cb) = on_change {
            if let Ok(mut f) = cb.lock() { f(val); }
        }
        KairosEvent::None
    })
        .step(s.step)
        .style(move |_, status| iced::widget::slider::Style {
            rail: iced::widget::slider::Rail {
                backgrounds: (
                    iced::Background::Color(fill_color),
                    iced::Background::Color(track_color),
                ),
                width: 4.0,
                border: iced::Border::default(),
            },
            handle: iced::widget::slider::Handle {
                shape: iced::widget::slider::HandleShape::Circle { radius: 6.0 },
                background: iced::Background::Color(handle_color),
                border_width: 0.0,
                border_color: iced::Color::TRANSPARENT,
            },
        })
        .into()
}