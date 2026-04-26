use iced::Background;
use iced::widget::container;
use mantle::palette::Palette;
use mantle::types::theme::ThemeSet;
use mantle::line::LineStyle;
use petra::primitives::divider::Divider;
use petra::primitives::types::Orientation;
use crate::conversion::KairosEvent;
use crate::conversion::style::resolve_color_source;

pub fn convert_divider(d: Divider, p: &Palette, theme: &ThemeSet) -> iced::Element<'static, KairosEvent> {
    let color = resolve_color_source(&d.color, p);

    match d.style {
        LineStyle::Solid => {
            match d.orientation {
                Orientation::Horizontal => {
                    container(iced::widget::Space::new(iced::Length::Fill, iced::Length::Fixed(d.thickness)))
                        .width(iced::Length::Fill)
                        .height(iced::Length::Fixed(d.thickness))
                        .style(move |_| container::Style {
                            background: Some(Background::Color(color)),
                            ..Default::default()
                        })
                        .into()
                }
                Orientation::Vertical => {
                    container(iced::widget::Space::new(iced::Length::Fixed(d.thickness), iced::Length::Fill))
                        .width(iced::Length::Fixed(d.thickness))
                        .height(iced::Length::Fill)
                        .style(move |_| container::Style {
                            background: Some(Background::Color(color)),
                            ..Default::default()
                        })
                        .into()
                }
            }
        }
        LineStyle::Repeated(shape, count) => {
            use iced::widget::canvas::{self, Frame, Geometry, Program};
            use iced::{Rectangle, Renderer, Theme};

            struct RepeatedDivider {
                shape: mantle::shape::Shape,
                count: u8,
                color: iced::Color,
                thickness: f32,
                horizontal: bool,
            }

            impl Program<KairosEvent> for RepeatedDivider {
                type State = ();
                fn draw(&self, _state: &(), renderer: &Renderer, _theme: &Theme, bounds: Rectangle, _cursor: iced::mouse::Cursor) -> Vec<Geometry> {
                    let mut frame = Frame::new(renderer, bounds.size());
                    let count = self.count.max(1) as f32;
                    for i in 0..self.count {
                        let t = (i as f32 + 0.5) / count;
                        let (x, y) = if self.horizontal {
                            (bounds.width * t, bounds.height / 2.0)
                        } else {
                            (bounds.width / 2.0, bounds.height * t)
                        };
                        crate::conversion::spinner::shapes::draw_shape(
                            &mut frame,
                            &self.shape,
                            iced::Point::new(x, y),
                            self.thickness * 2.0,
                            self.color,
                            0.0,
                        );
                    }
                    vec![frame.into_geometry()]
                }
            }

            let horizontal = matches!(d.orientation, Orientation::Horizontal);
            let program = RepeatedDivider { shape, count, color, thickness: d.thickness, horizontal };

            if horizontal {
                iced::widget::canvas(program)
                    .width(iced::Length::Fill)
                    .height(iced::Length::Fixed(d.thickness * 4.0))
                    .into()
            } else {
                iced::widget::canvas(program)
                    .width(iced::Length::Fixed(d.thickness * 4.0))
                    .height(iced::Length::Fill)
                    .into()
            }
        }
    }
}