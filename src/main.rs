use iced::{Element, Settings, Sandbox};
use iced::widget::{Container, column, row, Slider, Text, TextInput};
//use iced::Length::FillPortion;

use eval::{Value, Expr, to_value};

#[cfg(test)]
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
    res: String,
}

impl Sandbox for FCalc {
    type Message = Message;

    fn new() -> FCalc {
        FCalc {
            expr: String::from(""),
            arg: 0.0,
            arg_str: String::from(""),
            res: String::from(""),
        }
    }

    fn title(&self) -> String {
        String::from("this is ass")
    }

    fn update(&mut self, msg: Message) {
        match msg {
            Message::ArgChange(arg) => {self.arg = arg; self.arg_str = self.arg.to_string()},
            Message::ExprChange(f) => {self.expr = f},
        }
        self.calculate();
    }

    fn view(&self) -> Element<Message> {
        let slider_arg = Slider::new(-100.0..=100.0, self.arg, Message::ArgChange);
        let arg_out = Text::new(&*self.arg_str.as_str());
        let result_out = Text::new(&self.res);
        let expr_in = TextInput::new("Enter function", &self.expr, Message::ExprChange);
        Container::new(
            column![result_out, slider_arg, row![expr_in, arg_out]]
        ).center_x().center_y().into()
    }
}

impl FCalc {
    fn extract_value(n: Vec<Value>) -> f64 {
        to_value(n.get(0).unwrap()).as_f64().unwrap()
    }

    fn calculate(&mut self) {
        let processed_expr = Expr::new(&self.expr)
            .function("sin", |n| Ok(to_value(FCalc::extract_value(n).sin())))
            .function("cos", |n| Ok(to_value(FCalc::extract_value(n).cos())))
            .function("tan", |n| Ok(to_value(FCalc::extract_value(n).tan())))
            .value("x", &self.arg);
        let expr_result = processed_expr.exec();
        self.res = match expr_result {
            Ok(res) => res.to_string(),
            Err(_) => String::from("Error during computation"),
        }
    }
}

