use iced::widget::canvas::{
    Cursor, Frame, Geometry, LineCap, LineDash, LineJoin, Path, Program, Stroke, Style,
};
use iced::{Color, Point, Rectangle, Theme};

// shamelessly stolen from iced-rs api reference lol
#[derive(Debug)]
pub struct Graph {
    points: Vec<Point>,
    scale: f32,
    sliding_point: Point,
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

        let stroke_axis = Stroke {
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

        let stroke_grid = Stroke {
            style: Style::Solid(Color {
                r: 1.0,
                g: 0.0,
                b: 0.0,
                a: 0.5,
            }),
            width: 1.5,
            line_cap: LineCap::Round,
            line_join: LineJoin::Bevel,
            line_dash: LineDash::default(),
        };

        frame.stroke(&Path::line(points[0], points[2]), stroke_axis.clone());
        frame.stroke(&Path::line(points[1], points[3]), stroke_axis.clone());

        let grid_width = (frame.width() as f32 / self.scale) as i32;

        // i should probably stop redrawing axis and grid every time it plots a graph, so TODO
        for i in (-1) * grid_width..=grid_width {
            frame.stroke(
                &Path::line(
                    Point::new(0.0, frame.center().y + i as f32 * self.scale),
                    Point::new(frame.width(), frame.center().y + i as f32 * self.scale),
                ),
                stroke_grid.clone(),
            );
            frame.stroke(
                &Path::line(
                    Point::new(frame.center().x + i as f32 * self.scale, 0.0),
                    Point::new(frame.center().x + i as f32 * self.scale, frame.height()),
                ),
                stroke_grid.clone(),
            );
        }

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
        frame.fill(
            &Path::circle(
                Point::new(
                    frame.center().x + self.scale * self.sliding_point.x,
                    frame.center().y - self.scale * self.sliding_point.y,
                ),
                self.scale * 0.25,
            ),
            Color {
                r: 1.0,
                g: 0.0,
                b: 1.0,
                a: 0.75,
            },
        );
        vec![frame.into_geometry()]
    }
}
impl Graph {
    pub fn new(p: Vec<Point>, fl: f32, sliding_point: Point) -> Graph {
        Graph {
            points: p,
            scale: fl,
            sliding_point: sliding_point,
        }
    }

    pub fn add_point(&mut self, p: Point) {
        self.points.push(p);
    }

    pub fn set_sliding_point(&mut self, p: Point) {
        self.sliding_point = p;
    }
}
