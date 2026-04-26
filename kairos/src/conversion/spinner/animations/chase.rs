use crate::conversion::spinner::SpinnerProgram;
use crate::conversion::spinner::shapes::draw_shape;

pub fn draw_chase(
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

    let base_angle = phase * std::f32::consts::TAU * dir;
    let trail_steps = 10;

    for shape_i in 0..program.count {
        let shape_offset = shape_i as f32 / program.count as f32;
        let shape_angle = base_angle + shape_offset * std::f32::consts::TAU * dir;
        let x = cx + radius * shape_angle.cos();
        let y = cy + radius * shape_angle.sin();
        draw_shape(frame, &program.shape, iced::Point::new(x, y), dot_size, program.color, shape_angle);

        let trail_length: f32 = 0.15;
        let max_trail = (1.0 / program.count as f32) * 0.7;
        let actual_trail = trail_length.min(max_trail);

        for t in 1..=trail_steps {
            let t_frac = t as f32 / trail_steps as f32;
            let trail_angle = shape_angle - t_frac * actual_trail * std::f32::consts::TAU * dir;
            let glow = (1.0 - t_frac).powi(2) * 0.4;
            let trail_size = dot_size * (1.0 - t_frac * 0.5);
            let tx = cx + radius * trail_angle.cos();
            let ty = cy + radius * trail_angle.sin();
            let glow_color = iced::Color {
                a: program.color.a * glow,
                ..program.color
            };
            draw_shape(frame, &program.shape, iced::Point::new(tx, ty), trail_size, glow_color, trail_angle);
        }
    }
}