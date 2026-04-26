use crate::conversion::spinner::SpinnerProgram;
use crate::conversion::spinner::shapes::draw_shape;

pub fn draw_wave(
    frame: &mut iced::widget::canvas::Frame,
    program: &SpinnerProgram,
    cx: f32, cy: f32,
    _radius: f32,
    dot_size: f32,
    phase: f32,
) {
    let dir = match program.direction {
        petra::Direction::Forward => 1.0_f32,
        petra::Direction::Reverse => -1.0_f32,
    };

    let count = program.count as f32;
    let spacing = program.size * 0.28;
    let total_width = (count - 1.0) * spacing;
    let amplitude = program.size * 0.25;

    for i in 0..program.count {
        let offset = i as f32 / count;
        let shape_phase = ((phase - offset * 0.3) * dir).rem_euclid(1.0);
        // Sine wave: up and down
        let wave = (shape_phase * std::f32::consts::TAU).sin();
        let x = cx - total_width / 2.0 + i as f32 * spacing;
        let y = cy + wave * amplitude;
        draw_shape(frame, &program.shape, iced::Point::new(x, y), dot_size, program.color, 0.0);
    }
}