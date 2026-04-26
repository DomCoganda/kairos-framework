use crate::conversion::spinner::SpinnerProgram;
use crate::conversion::spinner::shapes::draw_shape;

pub fn draw_morph(
    frame: &mut iced::widget::canvas::Frame,
    program: &SpinnerProgram,
    cx: f32, cy: f32,
    _radius: f32,
    dot_size: f32,
    phase: f32,
) {
    let shapes = match &program.motion {
        petra::SpinnerMotion::Morph(s) => s.clone(),
        _ => vec![],
    };
    if shapes.is_empty() {
        draw_shape(frame, &mantle::shape::Shape::Dot, iced::Point::new(cx, cy), dot_size, program.color, 0.0);
        return;
    }
    let index = (phase * shapes.len() as f32) as usize % shapes.len();
    draw_shape(frame, &shapes[index], iced::Point::new(cx, cy), dot_size, program.color, phase * std::f32::consts::TAU);
}