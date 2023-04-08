use iced::{Color, Element, Length, Renderer, Settings, Sandbox};
use iced::widget::{Container, Column, Slider, Text, TextInput};
use iced::theme;
use iced::alignment;

use eval::{Expr, eval, to_value};

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
    res: String,
}

impl Sandbox for FCalc {
    type Message = Message;

    fn new() -> FCalc {
        FCalc {
            expr: String::from(""),
            arg: 0.0,
            res: String::from(""),
        }
    }

    fn title(&self) -> String {
        String::from("this is ass")
    }

    fn update(&mut self, msg: Message) {
        match msg {
            Message::ArgChange(arg) => {self.arg = arg},
            Message::ExprChange(f) => {self.expr = f},
        }
        self.calculate();
    }

    fn view(&self) -> Element<Message> {
        let slider_arg = Slider::new(-100.0..=100.0, self.arg, Message::ArgChange);
        let result_out = Text::new(&self.res);
        let expr_in = TextInput::new("Enter function", &self.expr, Message::ExprChange);
        Container::new(Column::new().push(result_out).push(slider_arg).push(expr_in))
            .center_x().center_y().into()
    }
}

impl FCalc {
    fn calculate(&mut self) {
        //let mut processed_expr = &self.expr.replace("x", &(format!("({})", &self.arg.to_string())));
        let processed_expr = Expr::new(&self.expr
                                       .replace("sin(x)", "x.sin()")
                                       .replace("cos(x)", "x.cos()")
                                       .replace("tan(x)", "x.tan()"))
            .value("x", &self.arg);
        let expr_result = processed_expr.exec();
        println!("{:?}", expr_result);
        self.res = match expr_result {
            Ok(res) => res.to_string(),
            Err(_) => String::from("Error during computation"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn quadr() {
        let mut fc = FCalc::new();
        fc.expr = String::from("x * x + 4");
        fc.arg = 6.0;
        fc.calculate();
        assert_eq!(fc.res, String::from("40.0"));
    }

    #[test]
    fn quadr_2() {
        let mut fc = FCalc::new();
        fc.expr = String::from("x * x + 4");
        fc.arg = -6.0;
        fc.calculate();
        assert_eq!(fc.res, String::from("40.0"));
    }

    #[test]
    fn cubic() {
        let mut fc = FCalc::new();
        fc.expr = String::from("x * x * x + 4");
        fc.arg = -2.0;
        fc.calculate();
        assert_eq!(fc.res, String::from("-4.0"));
    }

    #[test]
    fn linear() {
        let mut fc = FCalc::new();
        fc.expr = String::from("x + 4");
        fc.arg = -10.0;
        fc.calculate();
        assert_eq!(fc.res, String::from("-6.0"));
    }

    #[test]
    fn trigonometry() {
        let mut fc = FCalc::new();
        fc.expr = String::from("cos(x) * cos(x) + sin(x) * sin(x)");
        fc.arg = 2.11;
        fc.calculate();
        assert!(fc.res.parse::<f64>().unwrap() - 1.0 < 0.1);
    }
}
