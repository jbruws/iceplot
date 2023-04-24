use iced::Point;

use crate::graph_tool::GraphHandler;
use eval::{to_value, Expr, Value};

pub struct ExprCalculator {
    pub expr: String,
    pub arg: f64,
}

impl ExprCalculator {
    pub fn new() -> ExprCalculator {
        ExprCalculator {
            expr: String::new(),
            arg: 0.0,
        }
    }

    pub fn create_graph(&self) -> GraphHandler {
        const SCALE: f64 = 30.0;
        const PRECISION: f64 = 0.15;

        let mut initial_point = Point::new(0.0, 0.0);
        if let Ok(current_y) = ExprCalculator::calculate(&self.arg, &self.expr) {
            initial_point = Point::new(self.arg as f32, current_y as f32);
        }

        let mut gr = GraphHandler::new(Vec::new(), SCALE as f32, initial_point);
        let mut i = -50.0;
        while i < 50.0 {
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
            Err(msg) => return msg,
        }
    }

    pub fn extract_float(n: Vec<Value>) -> f64 {
        let return_float = match n.get(0) {
            Some(value) => match value.as_f64() {
                Some(f) => f,
                None => 0.0,
            },
            None => 0.0,
        };
        return_float
    }

    pub fn calculate(arg: &f64, expr: &String) -> Result<f64, String> {
        let processed_expr = Expr::new(expr)
            .function("sin", |n| {
                Ok(to_value(ExprCalculator::extract_float(n).sin()))
            })
            .function("cos", |n| {
                Ok(to_value(ExprCalculator::extract_float(n).cos()))
            })
            .function("tan", |n| {
                Ok(to_value(ExprCalculator::extract_float(n).tan()))
            })
            .function("ctg", |n| {
                Ok(to_value(1.0 / ExprCalculator::extract_float(n).tan()))
            })
            .function("sqrt", |n| {
                Ok(to_value(ExprCalculator::extract_float(n).powf(0.5)))
            })
            .function("abs", |n| {
                Ok(to_value(ExprCalculator::extract_float(n).abs()))
            })
            .value("x", arg);
        let expr_result = processed_expr.exec();
        match expr_result {
            Ok(res) => match res.as_f64() {
                Some(f) => Ok(f),
                // make actual error msgs later
                None => Err(String::from("Incomplete function")),
            },
            Err(_) => Err(String::from("Computation error (check syntax)")),
        }
    }
}
