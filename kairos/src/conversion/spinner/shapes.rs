use mantle::shape::Shape;

const DOT_SVG: &[u8] = include_bytes!("../../assets/icon/dot.svg");
const ARC_SVG: &[u8] = include_bytes!("../../assets/icon/arc.svg");
const LINE_SVG: &[u8] = include_bytes!("../../assets/icon/line.svg");
const SQUARE_SVG: &[u8] = include_bytes!("../../assets/icon/square.svg");
const TRIANGLE_SVG: &[u8] = include_bytes!("../../assets/icon/triangle.svg");

fn colorize_svg(bytes: &[u8], color: iced::Color) -> Vec<u8> {
    let hex = format!("#{:02X}{:02X}{:02X}",
                      (color.r * 255.0) as u8,
                      (color.g * 255.0) as u8,
                      (color.b * 255.0) as u8,
    );
    let svg = std::str::from_utf8(bytes).unwrap_or("");
    svg.replace("currentColor", &hex).into_bytes()
}

pub fn draw_shape(
    frame: &mut iced::widget::canvas::Frame,
    shape: &Shape,
    center: iced::Point,
    size: f32,
    color: iced::Color,
    angle: f32,
) {
    use iced::widget::canvas::{Fill, Path, Stroke, Style};

    let half = size / 2.0;

    frame.with_save(|frame| {
        frame.translate(iced::Vector::new(center.x, center.y));
        frame.rotate(angle);

        match shape {
            Shape::Dot => {
                let path = Path::new(|b| b.circle(iced::Point::ORIGIN, half));
                frame.fill(&path, Fill { style: Style::Solid(color), ..Default::default() });
            }
            Shape::Square => {
                let path = Path::rectangle(
                    iced::Point::new(-half, -half),
                    iced::Size::new(size, size),
                );
                frame.fill(&path, Fill { style: Style::Solid(color), ..Default::default() });
            }
            Shape::Line => {
                let path = Path::new(|b| {
                    b.move_to(iced::Point::new(0.0, -half));
                    b.line_to(iced::Point::new(0.0, half));
                });
                frame.stroke(&path, Stroke {
                    style: Style::Solid(color),
                    width: (size * 0.25).max(1.0),
                    line_cap: iced::widget::canvas::LineCap::Round,
                    ..Default::default()
                });
            }
            Shape::Triangle => {
                let path = Path::new(|b| {
                    b.move_to(iced::Point::new(0.0, -half));
                    b.line_to(iced::Point::new(half, half));
                    b.line_to(iced::Point::new(-half, half));
                    b.close();
                });
                frame.fill(&path, Fill { style: Style::Solid(color), ..Default::default() });
            }
            Shape::Arc => {
                let path = Path::new(|b| {
                    b.arc(iced::widget::canvas::path::Arc {
                        center: iced::Point::ORIGIN,
                        radius: half,
                        start_angle: iced::Radians(0.0),
                        end_angle: iced::Radians(std::f32::consts::PI),
                    });
                });
                frame.stroke(&path, Stroke {
                    style: Style::Solid(color),
                    width: (size * 0.2).max(1.0),
                    line_cap: iced::widget::canvas::LineCap::Round,
                    ..Default::default()
                });
            }
            Shape::Custom(_) => {
                let path = Path::new(|b| b.circle(iced::Point::ORIGIN, half));
                frame.fill(&path, Fill { style: Style::Solid(color), ..Default::default() });
            }
        }
    });
}