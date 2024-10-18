use crate::structs::expression::{Expr, Operator};

pub fn evaluate(expr: &Expr) -> (f64, Vec<String>) {
    match expr {
        Expr::Number(n) => (*n, vec![n.to_string()]),
        Expr::Op(left, op, right) => {
            let (left_val, mut left_steps) = evaluate(left);
            let (right_val, mut right_steps) = evaluate(right);

            let result = match op {
                Operator::Add => left_val + right_val,
                Operator::Subtract => left_val - right_val,
                Operator::Multiply => left_val * right_val,
                Operator::Divide => left_val / right_val,
                Operator::Potentiation => {
                    if left_val < 0.0 {
                        -left_val.powf(right_val)
                    } else {
                        left_val.powf(right_val)
                    }
                }
                Operator::CalculateRoot => {
                    if left_val.is_sign_negative() {
                        return (0.0, vec!["Raiz negativa".to_string()]);
                    }
                    left_val.powf(1.0 / right_val)
                }
            };

            let step = format!("{} {} {} = {}", left_val, op.to_string(), right_val, result);
            left_steps.append(&mut right_steps);
            left_steps.push(step);
            (result, left_steps)
        }
    }
}
