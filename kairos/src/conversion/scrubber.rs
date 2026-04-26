use mantle::types::theme::ThemeSet;
use mantle::palette::Palette;
use petra::primitives::scrubber::Scrubber;
use crate::conversion::KairosEvent;
use crate::conversion::style::{resolve_color_source, to_iced_length};

pub(crate) fn convert_scrubber(s: Scrubber, palette: &Palette, theme: &ThemeSet) -> iced::Element<'static, KairosEvent> {
    let current = s.current_frame.get() as f32;
    let total = s.total_frames as f32;
    let sig = s.current_frame.clone();
    let on_seek = s.on_seek.clone();
    let fill_color = resolve_color_source(&s.fill_color, palette);
    let track_color = resolve_color_source(&s.track_color, palette);
    let handle_color = resolve_color_source(&s.handle_color, palette);
    let width = to_iced_length(s.width, &theme.sizes);
    let height = s.height;

    iced::widget::slider(0.0f32..=total.max(1.0), current, move |val| {
        let frame = val as u32;
        sig.set(frame);
        if let Some(ref cb) = on_seek {
            if let Ok(mut f) = cb.lock() { f(frame); }
        }
        KairosEvent::None
    })
        .width(width)
        .style(move |_, _| iced::widget::slider::Style {
            rail: iced::widget::slider::Rail {
                backgrounds: (
                    iced::Background::Color(fill_color),
                    iced::Background::Color(track_color),
                ),
                width: height,
                border: iced::Border::default(),
            },
            handle: iced::widget::slider::Handle {
                shape: iced::widget::slider::HandleShape::Circle { radius: height + 2.0 },
                background: iced::Background::Color(handle_color),
                border_width: 0.0,
                border_color: iced::Color::TRANSPARENT,
            },
        })
        .into()
}