use iced::widget::canvas::{Path, Stroke, Style};
use crate::conversion::spinner::SpinnerProgram;

pub fn draw_ripple(
    frame: &mut iced::widget::canvas::Frame,
    program: &SpinnerProgram,
    cx: f32, cy: f32,
    radius: f32,
    _dot_size: f32,
    phase: f32,
) {
    let count = program.count.max(1) as f32;
    for i in 0..program.count {
        let offset = i as f32 / count;
        let ring_phase = (phase + offset).rem_euclid(1.0);
        let r = radius * ring_phase;
        let alpha = program.color.a * (1.0 - ring_phase);
        let color = iced::Color { a: alpha, ..program.color };
        let path = Path::new(|b| {
            b.arc(iced::widget::canvas::path::Arc {
                center: iced::Point::new(cx, cy),
                radius: r,
                start_angle: iced::Radians(0.0),
                end_angle: iced::Radians(std::f32::consts::TAU),
            });
        });
        frame.stroke(&path, Stroke {
            style: Style::Solid(color),
            width: 1.5,
            ..Default::default()
        });
    }
}