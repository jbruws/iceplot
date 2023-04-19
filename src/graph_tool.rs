use iced::widget::canvas::path::Builder;
use iced::widget::canvas::{Cursor, Frame, Geometry, Program};
use iced::{Color, Point, Rectangle, Theme};

// shamelessly stolen from iced-rs api reference lol
#[derive(Debug)]
pub struct Graph {
    points: Vec<Point>,
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

        let mut builder = Builder::new();
        builder.move_to(Point::new(frame.center().x, frame.center().y - 200.0));

        for i in &self.points {
            builder.line_to(*i);
        }
        for i in self.points.iter().rev() {
            builder.line_to(Point::new(i.x, i.y + 5.0));
        }
        builder.close();
        let final_path = builder.build();
        frame.fill(&final_path, Color::BLACK);
        vec![frame.into_geometry()]
    }
}
impl Graph {
    pub fn new(p: Vec<Point>) -> Graph {
        Graph { points: p }
    }
}
