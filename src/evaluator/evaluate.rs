use crate::structs::expression::{Expr, Operator};

fn calcula_raiz(num: f64, n: u32) -> f64 {
    if num < 0.0 && n % 2 == 0 {
        // Para raízes pares de números negativos, retornar a parte positiva e imprimir imaginário
        let raiz_imaginaria = (-num).powf(1.0 / n as f64);
        println!("Raiz {} de {} é {}i", n, num, raiz_imaginaria);
        raiz_imaginaria
    } else {
        // Para raízes ímpares ou números positivos
        num.powf(1.0 / n as f64)
    }
}

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
                Operator::CalculateRoot => calcula_raiz(left_val, right_val as u32),
            };

            let step = format!("{} {} {} = {}", left_val, op.to_string(), right_val, result);
            left_steps.append(&mut right_steps);
            left_steps.push(step);
            (result, left_steps)
        }
    }
}
