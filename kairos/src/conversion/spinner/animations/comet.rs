use crate::conversion::spinner::SpinnerProgram;
use crate::conversion::spinner::shapes::draw_shape;

pub fn draw_comet(
    frame: &mut iced::widget::canvas::Frame,
    program: &SpinnerProgram,
    cx: f32, cy: f32,
    radius: f32,
    dot_size: f32,
    phase: f32,
) {
    let dir = match program.direction {
        petra::Direction::Forward => 1.0_f32,
        petra::Direction::Reverse => -1.0_f32,
    };

    let base_angle = (program.time / program.speed) * std::f32::consts::TAU * dir;
    let trail_steps = 12;

    for shape_i in 0..program.count {
        let shape_offset = shape_i as f32 / program.count as f32;
        let shape_angle = base_angle + shape_offset * std::f32::consts::TAU * dir;

        // Draw trail first so lead shape renders on top
        let trail_length: f32 = (1.0 / (program.count as f32 * 2.0)).min(0.12);
        let actual_trail = trail_length;

        for t in 1..=trail_steps {
            let t_frac = t as f32 / trail_steps as f32;
            let trail_angle = shape_angle - t_frac * actual_trail * std::f32::consts::TAU * dir;
            let glow = (1.0 - t_frac).powi(2) * 0.5;
            let trail_size = dot_size * 0.6 * (1.0 - t_frac * 0.7);
            let tx = cx + radius * trail_angle.cos();
            let ty = cy + radius * trail_angle.sin();
            let trail_path = iced::widget::canvas::Path::new(|b| {
                b.circle(iced::Point::new(tx, ty), trail_size);
            });
            frame.fill(&trail_path, iced::Color {
                a: program.color.a * glow,
                ..program.color
            });
        }

        // Lead shape at full color on top
        let x = cx + radius * shape_angle.cos();
        let y = cy + radius * shape_angle.sin();
        draw_shape(frame, &program.shape, iced::Point::new(x, y), dot_size, program.color, shape_angle);
    }
}