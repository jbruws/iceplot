use iced::Point;

use crate::graph_tool::GraphHandler;
use evalexpr::*;
use num_cpus;
use std::sync::mpsc;
use std::thread;

pub struct ExprCalculator {
    pub expr: String,
    pub arg: f32,
    pub graph_scale: f32,
}

impl ExprCalculator {
    pub fn new() -> ExprCalculator {
        ExprCalculator {
            expr: String::new(),
            arg: 0.0,
            graph_scale: 30.0,
        }
    }

    pub fn create_graph(&self) -> GraphHandler {
        const PRECISION: f32 = 0.05; // larger = rougher graph

        let mut initial_point = Point::new(0.0, 0.0);
        if let Ok(current_y) = ExprCalculator::calculate(&self.arg, &self.expr) {
            if current_y.is_finite() {
                initial_point = Point::new(self.arg as f32, current_y as f32);
            }
        }

        let (tx, rx) = mpsc::channel();
        let cpu_count = num_cpus::get();
        let graph_width = (60.0 - self.graph_scale) * 3.0; // 50 (max scale) + 10 (min scale)
        let thread_range_size = graph_width / cpu_count as f32;

        for i in 0..cpu_count {
            // bounds of argument range for a particular thread
            // `lower_bound` serves as an argument and is incremented each loop pass
            let mut lower_bound = -0.5 * graph_width + thread_range_size * i as f32;
            let upper_bound = lower_bound + thread_range_size as f32;

            let expr_clone = self.expr.clone();
            let tx_clone = tx.clone();

            thread::spawn(move || {
                let mut result_vec = Vec::new();
                while lower_bound < upper_bound {
                    let function_val = ExprCalculator::calculate(&lower_bound, &expr_clone);
                    if let Ok(res) = function_val {
                        result_vec.push(Point::new(lower_bound as f32, res as f32));
                    }
                    lower_bound += &PRECISION;
                }
                tx_clone.send((i as i32, result_vec)).unwrap();
            });
        }

        drop(tx);
        let mut combined_segments = Vec::new();
        for received_segments in rx {
            combined_segments.push(received_segments);
        }

        combined_segments.sort_by_key(|i| i.0); // sorting by X coordinate to get a continous graph

        let mut gr = GraphHandler::new(Vec::new(), self.graph_scale, initial_point);
        for t in combined_segments {
            gr.add_points(&t.1);
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
            "tan" => Function::new(|n| {
                Ok(Value::Float(n.as_number()?.tan()))
            }),
            "cot" => Function::new(|n| {
                Ok(Value::Float(1.0 / n.as_number()?.tan()))
            }),
            "sqrt" => Function::new(|n| {
                Ok(Value::Float(n.as_number()?.sqrt()))
            }),
            "abs" => Function::new(|n| {
                Ok(Value::Float(n.as_number()?.abs()))
            }),
            "x" => arg,
            "pi" => std::f64::consts::PI,
            "e" => std::f64::consts::E,
        };

        let res = match ctx {
            // this whole block is kinda awful
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
