use iced::widget::canvas::{self, Frame, Geometry, Path, Program};
use iced::{ Rectangle, Renderer, Theme};
use mantle::palette::Palette;
use mantle::types::theme::ThemeSet;
use petra::Graph;
use crate::conversion::style::to_iced_length;
use crate::conversion::KairosEvent;

struct GraphProgram {
    data: Vec<f32>,
    min: f32,
    max: f32,
    line_color: iced::Color,
    fill_color: iced::Color,
    fill_opacity: f32,
    bg_color: iced::Color,
    bg_opacity: f32,
    line_width: f32,
}

impl Program<KairosEvent> for GraphProgram {
    type State = ();

    fn draw(&self, _state: &(), renderer: &Renderer, _theme: &Theme, bounds: Rectangle, _cursor: iced::mouse::Cursor) -> Vec<Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());

        let background = Path::rectangle(
            iced::Point::ORIGIN,
            bounds.size(),
        );
        frame.fill(&background, iced::widget::canvas::Fill {
            style: canvas::Style::Solid(iced::Color { a: self.bg_opacity, ..self.bg_color }),
            ..Default::default()
        });

        let n = self.data.len();
        if n < 2 {
            return vec![frame.into_geometry()];
        }

        let points: Vec<iced::Point> = self.data.iter().enumerate().map(|(i, &val)| {
            let x = i as f32 / (n - 1) as f32 * bounds.width;
            let y = bounds.height - ((val - self.min) / (self.max - self.min)) * bounds.height;
            iced::Point::new(x, y)
        }).collect();

        let fill_path = Path::new(|b| {
            b.move_to(iced::Point::new(0.0, bounds.height));
            for p in &points {
                b.line_to(*p);
            }
            b.line_to(iced::Point::new(bounds.width, bounds.height));
            b.close();
        });

        frame.fill(&fill_path, canvas::Fill {
            style: canvas::Style::Solid(iced::Color { a: self.fill_opacity, ..self.fill_color }),
            ..Default::default()
        });

        let line_path = Path::new(|b| {
            b.move_to(points[0]);
            for p in points.iter().skip(1) {
                b.line_to(*p);
            }
        });

        frame.stroke(&line_path, canvas::Stroke {
            style: canvas::Style::Solid(self.line_color),
            width: self.line_width,
            ..Default::default()
        });

        vec![frame.into_geometry()]
    }
}


pub(crate) fn convert_graph(c: Graph, palette: &Palette, theme: &ThemeSet) -> iced::Element<'static, super::KairosEvent> {
    let width = to_iced_length(c.width,&theme.sizes);
    let height = to_iced_length(c.height,&theme.sizes);

    let max = if c.max == 0.0 {
        c.data.iter().cloned().fold(0.0f32, f32::max).max(1.0)
    } else {
        c.max
    };

    let program = GraphProgram {
        data: c.data,
        min: c.min,
        max,
        line_color: super::style::resolve_color_source(&c.color, palette),
        fill_color: super::style::resolve_color_source(&c.fill_color, palette),
        fill_opacity: c.fill_opacity,
        bg_color: super::style::resolve_color_source(&c.bg_color, palette),
        bg_opacity: c.bg_opacity,
        line_width: c.line_width,
    };

    iced::widget::canvas(program)
        .width(width)
        .height(height)
        .into()
}