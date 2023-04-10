use iced::alignment::Horizontal;
use iced::widget::{column, row, Container, Slider, Text, TextInput};
use iced::Length::FillPortion;
use iced::{Element, Sandbox, Settings, Point};

use crate::graph_tool::Graph;
use eval::{to_value, Expr, Value};

mod graph_tool;
mod tests;

fn main() -> iced::Result {
    FCalc::run(Settings::default())
}

#[derive(Clone, Debug)]
enum Message {
    ArgChange(f64),
    ExprChange(String),
}

struct FCalc {
    expr: String,
    arg: f64,
    arg_str: String,
    res: f64,
    res_str: String,
}

impl Sandbox for FCalc {
    type Message = Message;

    fn new() -> FCalc {
        FCalc {
            expr: String::from(""),
            arg: 0.0,
            arg_str: String::from("0.0"),
            res: 0.0,
            res_str: String::from("0.0"),
        }
    }

    fn title(&self) -> String {
        String::from("FCalc")
    }

    fn update(&mut self, msg: Message) {
        match msg {
            Message::ArgChange(arg) => {
                self.arg = arg;
                self.arg_str = self.arg.to_string()
            }
            Message::ExprChange(f) => self.expr = f,
        }
        self.calculate();
    }

    fn view(&self) -> Element<Message> {
        let slider_arg = Slider::new(-100.0..=100.0, self.arg, Message::ArgChange);
        let arg_out = Text::new(format!("x = {}", &*self.arg_str.as_str()))
            .width(FillPortion(1))
            .horizontal_alignment(Horizontal::Center);
        let result_out = Text::new(&self.res_str).horizontal_alignment(Horizontal::Center);
        let expr_in =
            TextInput::new("Enter function", &self.expr, Message::ExprChange).width(FillPortion(5));
        Container::new(
            column![result_out, slider_arg, row![expr_in, arg_out]]
                .padding(10)
                .spacing(10),
        )
        .center_x()
        .center_y()
        .into()
    }
}

impl FCalc {
    fn extract_float(n: Vec<Value>) -> f64 {
        to_value(n.get(0).unwrap()).as_f64().unwrap()
    }

    fn calculate(&mut self) {
        let processed_expr = Expr::new(&self.expr)
            .function("sin", |n| Ok(to_value(FCalc::extract_float(n).sin())))
            .function("cos", |n| Ok(to_value(FCalc::extract_float(n).cos())))
            .function("tan", |n| Ok(to_value(FCalc::extract_float(n).tan())))
            .value("x", &self.arg);
        let expr_result = processed_expr.exec();
        match expr_result {
            Ok(res) => {
                self.res = res.as_f64().unwrap();
                self.res_str = format!("f(x) = {}", res.to_string())
            }
            Err(_) => self.res_str = String::from("Computation error"),
        }
    }
}
