use crate::conversion::spinner::SpinnerProgram;
use crate::conversion::spinner::shapes::draw_shape;

pub fn draw_orbit(
    frame: &mut iced::widget::canvas::Frame,
    program: &SpinnerProgram,
    cx: f32, cy: f32,
    radius: f32,
    dot_size: f32,
    _phase: f32,
) {
    let dir = match program.direction {
        petra::Direction::Forward => 1.0_f32,
        petra::Direction::Reverse => -1.0_f32,
    };

    // Draw faint track ring
    let track = iced::widget::canvas::Path::new(|b| {
        b.arc(iced::widget::canvas::path::Arc {
            center: iced::Point::new(cx, cy),
            radius,
            start_angle: iced::Radians(0.0),
            end_angle: iced::Radians(std::f32::consts::TAU),
        });
    });
    frame.stroke(&track, iced::widget::canvas::Stroke {
        style: iced::widget::canvas::Style::Solid(iced::Color {
            a: 0.15,
            ..program.color
        }),
        width: 1.0,
        ..Default::default()
    });

    // N shapes orbiting evenly spaced
    let base_angle = (program.time / program.speed) * std::f32::consts::TAU * dir;
    for i in 0..program.count {
        let offset = i as f32 / program.count as f32;
        let angle = base_angle + offset * std::f32::consts::TAU * dir;
        let x = cx + radius * angle.cos();
        let y = cy + radius * angle.sin();
        draw_shape(frame, &program.shape, iced::Point::new(x, y), dot_size, program.color, angle);
    }
}