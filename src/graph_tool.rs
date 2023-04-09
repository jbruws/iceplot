use iced::widget::canvas::{self, Path, path, Canvas, Fill, Frame, Cache};
use iced::Point;

pub struct Graph {
    pub graph_points: Vec<Point>,
}

impl Graph {
    pub fn new() -> Graph {
        Graph{graph_points: vec![Point::new(0.0, 0.0), Point::new(0.0, 0.0)]}
    }

    pub fn new_point(&mut self, x: f32, y: f32) {
        self.graph_points.push(Point::new(x, y));
    }

    pub fn draw_graph(&self) -> Canvas<Message, Theme, P> {
        let graph = path::Builder::new();
        graph.move_to(*self.graph_points.get(0).unwrap());
        for i in 1..self.graph_points.len() {
            graph.line_to(self.graph_points.get(i).unwrap);
        }
        let graph_path = graph.build();
        let cache: layer::Cache<Graph> = layer::Cache::new();
        Canvas::new().push(cache.with(graph_path))
    }
}
