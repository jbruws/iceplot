use iced::widget::canvas::{
    Cursor, Frame, Geometry, LineCap, LineDash, LineJoin, Path, Program, Stroke, Style,
};
use iced::{Color, Point, Rectangle, Theme};

#[derive(Debug)]
pub struct GraphHandler {
    points: Vec<Point>,
    scale: f32,
    sliding_point: Point,
}

impl<Message> Program<Message> for GraphHandler {
    type State = ();

    fn draw(
        &self,
        _state: &(),
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<Geometry> {
        let mut frame = Frame::new(bounds.size());

        // current value indicator
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

        vec![
            self.create_background(bounds),
            self.create_geometry(bounds),
            frame.into_geometry(),
        ]
    }
}

impl GraphHandler {
    fn create_background(&self, bounds: Rectangle) -> Geometry {
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
        frame.into_geometry()
    }

    fn create_geometry(&self, bounds: Rectangle) -> Geometry {
        let mut frame = Frame::new(bounds.size());
        let mut towards_x_axis = false;

        for i in 1..self.points.len() {
            let prev_point = *self.points.get(i - 1).unwrap();
            let current_point = *self.points.get(i).unwrap();

            // filtering out infinite numbers
            if !current_point.y.is_finite() || !prev_point.y.is_finite() {
                continue;
            }
            // breaks in the graph
            // if the signs are different and the graph doesn't cross x axis
            // it can only mean one thing
            if current_point.y * prev_point.y < 0.0 && !towards_x_axis {
                continue;
            }

            // graph direction
            if (current_point.y < prev_point.y && prev_point.y > 0.0)
                || (current_point.y > prev_point.y && prev_point.y < 0.0)
            {
                towards_x_axis = true;
            } else {
                towards_x_axis = false;
            }

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
                    width: 2.0,
                    line_cap: LineCap::Round,
                    line_join: LineJoin::Bevel,
                    line_dash: LineDash::default(),
                },
            );
        }
        frame.into_geometry()
    }

    pub fn new(p: Vec<Point>, scale: f32, sliding_point: Point) -> GraphHandler {
        GraphHandler {
            points: p,
            scale: scale,
            sliding_point: sliding_point,
        }
    }

    pub fn add_points(&mut self, p: &Vec<Point>) {
        self.points.extend(p);
    }
}
