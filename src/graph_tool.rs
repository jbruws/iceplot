use iced::widget::canvas::{
    Cursor, Frame, Geometry, LineCap, LineDash, LineJoin, Path, Program, Stroke, Style,
};
use iced::{Color, Point, Rectangle, Theme};

// shamelessly stolen from iced-rs api reference lol
#[derive(Debug)]
pub struct Graph {
    points: Vec<Point>,
    scale: f32,
}

impl<Message> Program<Message> for Graph {
    type State = ();

    fn draw(
        &self,
        _state: &(),
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<Geometry> {
        let mut frame = Frame::new(bounds.size());

        let points = [
            Point::new(frame.center().x, 0.0),
            Point::new(0.0, frame.center().y),
            Point::new(frame.center().x, frame.height()),
            Point::new(frame.width(), frame.center().y),
        ];

        let stroke = Stroke {
            style: Style::Solid(Color {
                r: 1.0,
                g: 0.0,
                b: 0.0,
                a: 1.0,
            }),
            width: 2.0,
            line_cap: LineCap::Round,
            line_join: LineJoin::Bevel,
            line_dash: LineDash::default(),
        };

        frame.stroke(&Path::line(points[0], points[2]), stroke.clone());
        frame.stroke(&Path::line(points[1], points[3]), stroke.clone());

        for i in 1..self.points.len() {
            let prev_point = *self.points.get(i - 1).unwrap();
            let current_point = *self.points.get(i).unwrap();
            let current_path = Path::line(
                Point::new(
                    frame.center().x + self.scale * prev_point.x,
                    frame.center().y - self.scale * prev_point.y,
                ),
                Point::new(
                    frame.center().x + self.scale * current_point.x,
                    frame.center().y - self.scale * current_point.y,
                ),
            );
            frame.stroke(
                &current_path,
                Stroke {
                    style: Style::Solid(Color::BLACK),
                    width: 3.0,
                    line_cap: LineCap::Round,
                    line_join: LineJoin::Bevel,
                    line_dash: LineDash::default(),
                },
            );
        }
        vec![frame.into_geometry()]
    }
}
impl Graph {
    pub fn new(p: Vec<Point>, fl: f32) -> Graph {
        Graph {
            points: p,
            scale: fl,
        }
    }

    pub fn add_point(&mut self, p: Point) {
        self.points.push(p);
    }
}
