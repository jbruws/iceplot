use iced::alignment::Horizontal;
use iced::widget::canvas::Canvas;
use iced::widget::{column, row, Container, Slider, Text, TextInput};
use iced::Length;
use iced::Length::FillPortion;
use iced::{executor, Command, Theme};
use iced::{Application, Element, Point, Settings};

use crate::graph_tool::Graph;
use eval::{to_value, Expr, Value};

mod graph_tool;
mod tests;

fn main() -> iced::Result {
    FuncHandler::run(Settings::default())
}

#[derive(Clone, Debug)]
enum Message {
    ArgChange(f64),
    ExprChange(String),
}

struct FuncHandler {
    expr: String,
    arg: f64,
    arg_str: String,
    res: f64,
    res_str: String,
}

impl Application for FuncHandler {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            FuncHandler {
                expr: String::from(""),
                arg: 0.0,
                arg_str: String::from("0.0"),
                res: 0.0,
                res_str: String::from("0.0"),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("FuncHandler")
    }

    fn update(&mut self, msg: Message) -> Command<Message> {
        match msg {
            Message::ArgChange(arg) => {
                self.arg = arg;
                self.arg_str = self.arg.to_string()
            }
            Message::ExprChange(f) => self.expr = f,
        }
        self.set_values();
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let slider_arg = Slider::new(-100.0..=100.0, self.arg, Message::ArgChange);
        let arg_out = Text::new(format!("x = {}", &*self.arg_str.as_str()))
            .width(FillPortion(1))
            .horizontal_alignment(Horizontal::Center);
        let result_out = Text::new(&self.res_str).horizontal_alignment(Horizontal::Center);
        let expr_in =
            TextInput::new("Enter function", &self.expr, Message::ExprChange).width(FillPortion(5));
        let gr_canvas = Canvas::new(self.create_graph())
            .width(Length::Fill)
            .height(Length::Fill);
        Container::new(
            column![result_out, slider_arg, row![expr_in, arg_out], gr_canvas]
                .padding(10)
                .spacing(10),
        )
        .center_x()
        .center_y()
        .into()
    }
}

impl FuncHandler {
    fn create_graph(&self) -> Graph {
        let mut graph_points: Vec<Point> = Vec::new();
        const SCALE: i32 = 10;
        for i in -100..=100 {
            let function_val = FuncHandler::calculate(self.expr.clone(), i as f64);
            if let Ok(res) = function_val {
                graph_points.push(Point::new(
                    (SCALE * i) as f32,
                    (SCALE as f64 * res) as f32,
                ));
            }
        }
        Graph::new(graph_points)
    }

    fn set_values(&mut self) {
        let current_value = FuncHandler::calculate(self.expr.clone(), self.arg);
        match current_value {
            Ok(res) => {
                self.res = res;
                self.res_str = format!("f(x) = {}", res.to_string())
            }
            Err(msg) => self.res_str = msg,
        }
    }

    fn extract_float(n: Vec<Value>) -> f64 {
        match to_value(n.get(0).unwrap()).as_f64() {
            // and now it crashes here????
            Some(f) => f,
            None => 0.0,
        }
    }

    fn calculate(expr: String, arg: f64) -> Result<f64, String> {
        let processed_expr = Expr::new(expr)
            .function("sin", |n| Ok(to_value(FuncHandler::extract_float(n).sin())))
            .function("cos", |n| Ok(to_value(FuncHandler::extract_float(n).cos())))
            .function("tan", |n| Ok(to_value(FuncHandler::extract_float(n).tan())))
            .value("x", arg);
        let expr_result = processed_expr.exec();
        match expr_result {
            Ok(res) => match res.as_f64() {
                Some(f) => Ok(f),
                None => Err(String::from("Incomplete function")),
            },
            Err(_) => Err(String::from("Computation error (check syntax)")),
        }
    }
}
