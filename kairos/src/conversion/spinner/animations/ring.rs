use mantle::shape::Shape;
use crate::conversion::spinner::SpinnerProgram;
use crate::conversion::spinner::shapes::draw_shape;


pub fn draw_ring(
    frame: &mut iced::widget::canvas::Frame,
    program: &SpinnerProgram,
    cx: f32, cy: f32,
    radius: f32,
    dot_size: f32,
    phase: f32,
) {
    let dir = match program.direction {
        petra::Direction::Forward => 1.0,
        petra::Direction::Reverse => -1.0,
    };

    match &program.shape {
        Shape::Arc => {
            let base_rotation = (program.time / program.speed) * 2.0 * std::f32::consts::PI * dir;
            let circumference = 2.0 * std::f32::consts::PI * radius;
            let count = program.count as f32;
            let max_len = (circumference / count) * 0.98;
            let arc_len = (1.0 - (phase * 2.0 - 1.0).abs()) * max_len;
            for i in 0..program.count {
                let offset = i as f32 / count;
                let start = base_rotation + offset * 2.0 * std::f32::consts::PI;
                let path = iced::widget::canvas::Path::new(|b| {
                    b.arc(iced::widget::canvas::path::Arc {
                        center: iced::Point::new(cx, cy),
                        radius,
                        start_angle: iced::Radians(start),
                        end_angle: iced::Radians(start + (arc_len / radius)),
                    });
                });
                frame.stroke(&path, iced::widget::canvas::Stroke {
                    style: iced::widget::canvas::Style::Solid(program.color),
                    width: dot_size,
                    line_cap: iced::widget::canvas::LineCap::Round,
                    ..Default::default()
                });
            }
        },
        _ => {
            let group_angle = (program.time / program.speed) * 2.0 * std::f32::consts::TAU * dir;
            let count = program.count as f32;
            for i in 0..program.count {
                let offset = i as f32 / count;
                let rise = ((phase - offset * 0.4) / 0.1).clamp(0.0, 1.0);
                let shrink_start = 0.5 + (1.0 - offset) * 0.4;
                let shrink = ((phase - shrink_start) / 0.1).clamp(0.0, 1.0);
                let brightness = (rise - shrink).max(0.0);
                let color = iced::Color {
                    a: (program.color.a * brightness).max(0.0),
                    ..program.color
                };
                let angle = group_angle + (offset * std::f32::consts::TAU);
                let x = cx + radius * angle.cos();
                let y = cy + radius * angle.sin();
                draw_shape(frame, &program.shape, iced::Point::new(x, y), dot_size, color, angle);
            }
        },
    }
}