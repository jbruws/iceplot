use iced::Point;

use crate::graph_tool::GraphHandler;
use evalexpr::*;

pub struct ExprCalculator {
    pub expr: String,
    pub arg: f32,
}

impl ExprCalculator {
    pub fn new() -> ExprCalculator {
        ExprCalculator {
            expr: String::new(),
            arg: 0.0,
        }
    }

    pub fn create_graph(&self) -> GraphHandler {
        const SCALE: f32 = 30.0;
        const PRECISION: f32 = 0.05; // still breaks at 1/x lmao

        let mut initial_point = Point::new(0.0, 0.0);
        if let Ok(current_y) = ExprCalculator::calculate(&self.arg, &self.expr) {
            initial_point = Point::new(self.arg as f32, current_y as f32);
        }

        let mut gr = GraphHandler::new(Vec::new(), SCALE, initial_point);
        let mut i = -30.0;
        while i < 30.0 {
            let function_val = ExprCalculator::calculate(&i, &self.expr);
            if let Ok(res) = function_val {
                gr.add_point(Point::new(i as f32, res as f32));
            }
            i += PRECISION;
        }
        gr
    }

    pub fn get_value(&self) -> String {
        let current_value = ExprCalculator::calculate(&self.arg, &self.expr);
        match current_value {
            Ok(res) => return res.to_string(),
            Err(msg) => return msg.to_string(),
        }
    }

    pub fn calculate(argm: &f32, expr: &String) -> Result<f64, EvalexprError> {
        let arg = *argm as f64;
        let ctx = context_map! {
            "sin" => Function::new(|n| {
                Ok(Value::Float(n.as_number()?.sin()))
            }),
            "cos" => Function::new(|n| {
                Ok(Value::Float(n.as_number()?.sin()))
            }),
            "tg" => Function::new(|n| {
                Ok(Value::Float(n.as_number()?.tan()))
            }),
            "ctg" => Function::new(|n| {
                Ok(Value::Float(1.0 / n.as_number()?.tan()))
            }),
            "sqrt" => Function::new(|n| {
                Ok(Value::Float(n.as_number()?.sqrt()))
            }),
            "abs" => Function::new(|n| {
                Ok(Value::Float(n.as_number()?.abs()))
            }),
            "x" => arg,
        };

        let res = match ctx {
            Ok(valid_ctx) => match eval_number_with_context(expr.as_str(), &valid_ctx) {
                Err(_) => Err(EvalexprError::CustomMessage(String::from(
                    "Computation error",
                ))),
                Ok(value) if !value.is_nan() => Ok(value),
                _ => Err(EvalexprError::CustomMessage(String::from(
                    "Undefined value",
                ))),
            },
            Err(_) => Err(EvalexprError::CustomMessage(String::from(
                "Computation error",
            ))),
        };
        res
    }
}
