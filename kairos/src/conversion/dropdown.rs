use mantle::types::theme::ThemeSet;
use mantle::palette::Palette;
use petra::composites::dropdown::Dropdown;
use crate::conversion::KairosEvent;

pub(crate) fn convert_dropdown(d: Dropdown, _palette: &Palette, _theme: &ThemeSet) -> iced::Element<'static, KairosEvent> {
    let sig = d.selected.clone();
    let selected = d.selected.get_clone();
    let options = d.options.clone();
    let placeholder = d.placeholder.clone();

    let pick = iced::widget::pick_list(
        options,
        selected,
        move |val: String| {
            sig.set(Some(val));
            KairosEvent::None
        },
    )
        .placeholder(placeholder)
        .width(iced::Length::Fill)
        .style(|_theme, status| {
            let is_open = matches!(status, iced::widget::pick_list::Status::Opened);
            iced::widget::pick_list::Style {
                background: iced::Background::Color(iced::Color::from_rgba(0.02, 0.02, 0.12, 0.95)),
                border: iced::Border {
                    color: iced::Color::from_rgba(0.54, 0.62, 0.69, if is_open { 0.4 } else { 0.15 }),
                    width: 1.0,
                    radius: iced::border::Radius::new(3.0),
                },
                text_color: iced::Color::from_rgb(0.95, 0.95, 0.97),
                placeholder_color: iced::Color::from_rgba(0.54, 0.62, 0.69, 0.7),
                handle_color: iced::Color::from_rgba(0.54, 0.62, 0.69, 0.7),
            }
        });

    pick.into()
}