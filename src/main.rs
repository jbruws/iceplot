use iced::alignment::Horizontal;
use iced::widget::canvas::Canvas;
use iced::widget::{column, row, Container, Slider, Text, TextInput, VerticalSlider};
use iced::Length;
use iced::Length::FillPortion;
use iced::{executor, Command, Theme};
use iced::{Application, Element, Settings};

use crate::expr_calculator::ExprCalculator;

mod expr_calculator;
mod graph_tool;
mod tests;

fn main() -> iced::Result {
    FuncHandler::run(Settings {
        antialiasing: true,
        ..Settings::default()
    })
}

#[derive(Clone, Debug)]
enum Message {
    ArgChange(f32),
    ExprChange(String),
    ScaleChange(f32),
}

struct FuncHandler {
    ecalc: ExprCalculator,
}

impl Application for FuncHandler {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            FuncHandler {
                ecalc: ExprCalculator::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("IcePlot")
    }

    fn update(&mut self, msg: Message) -> Command<Message> {
        match msg {
            Message::ArgChange(arg) => self.ecalc.arg = arg,
            Message::ExprChange(f) => self.ecalc.expr = f,
            Message::ScaleChange(s) => self.ecalc.graph_scale = s,
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let slider_arg = Slider::new(-30.0..=30.0, self.ecalc.arg, Message::ArgChange).step(0.1);
        let slider_scale =
            VerticalSlider::new(10.0..=50.0, self.ecalc.graph_scale, Message::ScaleChange)
                .step(1.0);
        let arg_out = Text::new(format!("x = {}", &self.ecalc.arg.to_string()))
            .width(FillPortion(1))
            .horizontal_alignment(Horizontal::Center);
        let result_out = Text::new(self.ecalc.get_value()).horizontal_alignment(Horizontal::Center);
        let expr_in = TextInput::new("Enter function", &self.ecalc.expr, Message::ExprChange)
            .width(FillPortion(5));
        let gr_canvas = Canvas::new(self.ecalc.create_graph())
            .width(Length::Fill)
            .height(Length::Fill);

        Container::new(
            column![
                result_out,
                slider_arg,
                row![expr_in, arg_out],
                row![gr_canvas, slider_scale].padding(5).spacing(5)
            ]
            .padding(10)
            .spacing(10),
        )
        .center_x()
        .center_y()
        .into()
    }
}
