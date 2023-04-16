use iced::widget::canvas::{self, Canvas, Cursor, Fill, Frame, Geometry, Path, Program};
use iced::{Color, Rectangle, Theme, Point};

// shamelessly stolen from iced-rs api reference lol
#[derive(Debug)]
pub struct Graph {
    points: Vec<Point>,
}

impl Program<()> for Graph {
    type State = ();

    fn draw(&self, _state: &(), _theme: &Theme, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
        let mut frame = Frame::new(bounds.size());
        let mut paths: Vec<Path> = Vec::new();
        let mut last_point: Point = frame.center();
        for i in 1..self.points.len() {
            let current_point = *self.points.get(i).unwrap();
            paths.push(Path::line(last_point, current_point));
            last_point = current_point;
        }
        vec![frame.into_geometry()]
    }
}
impl Graph {
    pub fn new(p: Vec<Point>) -> Graph {
        Graph{points: p}
    }
}
