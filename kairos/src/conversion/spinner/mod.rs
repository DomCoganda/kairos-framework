pub mod shapes;
pub mod animations;

use iced::{Rectangle, Renderer, Theme};
use iced::mouse::Cursor;
use iced::widget::canvas::{Geometry, Program};
use mantle::shape::Shape;
use petra::{Direction, SpinnerMotion};
use crate::conversion::KairosEvent;
use crate::conversion::style::resolve_color_source;

pub struct SpinnerProgram {
    pub shape: Shape,
    pub motion: SpinnerMotion,
    pub size: f32,
    pub color: iced::Color,
    pub speed: f32,
    pub direction: Direction,
    pub count: u8,
    pub time: f32,
}

impl Program<KairosEvent> for SpinnerProgram {
    type State = ();
    fn draw(&self, _state: &(), renderer: &Renderer, _theme: &Theme, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry<Renderer>> {
        let mut frame = iced::widget::canvas::Frame::new(renderer, bounds.size());
        let cx = bounds.width / 2.0;
        let cy = bounds.height / 2.0;
        let radius = self.size * 0.35;
        let dot_size = self.size * 0.12;
        let phase = (self.time / self.speed) % 1.0;

        match &self.motion {
            SpinnerMotion::Ring   => animations::ring::draw_ring(&mut frame, self, cx, cy, radius, dot_size, phase),
            SpinnerMotion::Chase  => animations::chase::draw_chase(&mut frame, self, cx, cy, radius, dot_size, phase),
            SpinnerMotion::Pulse  => animations::pulse::draw_pulse(&mut frame, self, cx, cy, radius, dot_size, phase),
            SpinnerMotion::Bounce => animations::bounce::draw_bounce(&mut frame, self, cx, cy, radius, dot_size, phase),
            SpinnerMotion::Grow   => animations::grow::draw_grow(&mut frame, self, cx, cy, radius, dot_size, phase),
            SpinnerMotion::Spin   => animations::spin::draw_spin(&mut frame, self, cx, cy, radius, dot_size, phase),
            SpinnerMotion::Orbit  => animations::orbit::draw_orbit(&mut frame, self, cx, cy, radius, dot_size, phase),
            SpinnerMotion::Wave   => animations::wave::draw_wave(&mut frame, self, cx, cy, radius, dot_size, phase),
            SpinnerMotion::Comet  => animations::comet::draw_comet(&mut frame, self, cx, cy, radius, dot_size, phase),
            SpinnerMotion::Ripple => animations::ripple::draw_ripple(&mut frame, self, cx, cy, radius, dot_size, phase),
            SpinnerMotion::Morph(_) => animations::morph::draw_morph(&mut frame, self, cx, cy, radius, dot_size, phase),
        }

        vec![frame.into_geometry()]
    }
}

pub fn convert_spinner(s: petra::Spinner, palette: &mantle::palette::Palette, theme: &mantle::types::theme::ThemeSet, time: f32) -> iced::Element<'static, KairosEvent> {
    let size = s.size.to_px(&theme.sizes);
    let color = resolve_color_source(&s.color, palette);
    let speed = match s.speed {
        mantle::animation::Animation::Slow   => 2.0,
        mantle::animation::Animation::Normal => 1.25,
        mantle::animation::Animation::Fast   => 0.6,
        mantle::animation::Animation::Custom(ms) => ms / 1000.0,
    };
    let program = SpinnerProgram {
        shape: s.shape,
        motion: s.motion,
        size,
        color,
        speed,
        direction: s.direction,
        count: s.count,
        time,
    };
    iced::widget::canvas(program)
        .width(iced::Length::Fixed(size))
        .height(iced::Length::Fixed(size))
        .into()
}