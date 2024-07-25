#[derive(Debug, PartialEq)]
pub enum Expr {
    Number(f64),
    Op(Box<Expr>, Operator, Box<Expr>),
}

#[derive(Debug, PartialEq)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Potentiation,
    CalculateRoot,
}

impl Operator {
    fn from_char(c: char) -> Option<Operator> {
        match c {
            '+' => Some(Operator::Add),
            '-' => Some(Operator::Subtract),
            '*' => Some(Operator::Multiply),
            '/' => Some(Operator::Divide),
            '^' => Some(Operator::Potentiation),
            'r' => Some(Operator::CalculateRoot),
            '(' => Some(Operator::Multiply),
            _ => None,
        }
    }

    fn to_string(&self) -> &str {
        match self {
            Operator::Add => "+",
            Operator::Subtract => "-",
            Operator::Multiply => "*",
            Operator::Divide => "/",
            Operator::Potentiation => "^",
            Operator::CalculateRoot => "r",
        }
    }

    fn precedence(&self) -> u8 {
        match self {
            Operator::Add | Operator::Subtract => 1,
            Operator::Multiply | Operator::Divide => 2,
            Operator::Potentiation | Operator::CalculateRoot => 3,
        }
    }
}

pub fn parse_expression(input: &str) -> Result<Expr, String> {
    let mut index = 0;
    let tokens: Vec<char> = input.chars().filter(|c| !c.is_whitespace()).collect();
    parse_expr(&tokens, &mut index, 0)
}

fn parse_expr(tokens: &[char], index: &mut usize, min_precedence: u8) -> Result<Expr, String> {
    let mut left = parse_term(tokens, index)?;

    while *index < tokens.len() {
        let op = match Operator::from_char(tokens[*index]) {
            Some(op) => op,
            None => break,
        };

        if op.precedence() < min_precedence {
            break;
        }
        *index += 1;
        let mut right = parse_term(tokens, index)?;

        while *index < tokens.len() {
            let next_op = match Operator::from_char(tokens[*index]) {
                Some(next_op) => next_op,
                None => break,
            };

            if next_op.precedence() <= op.precedence() {
                break;
            }

            if tokens[*index] == '^' {
                *index += 1;
                right = Expr::Op(
                    Box::new(right),
                    next_op,
                    Box::new(Expr::Number(
                        tokens[*index]
                            .to_digit(10)
                            .expect("Fail parse to number")
                            .into(),
                    )),
                );

                break;
            }

            if tokens[*index] == '*' || tokens[*index] == '/' {
                *index -= 1;
            }

            right = parse_expr(tokens, index, next_op.precedence())?;
        }

        left = Expr::Op(Box::new(left), op, Box::new(right));
    }

    Ok(left)
}

fn parse_term(tokens: &[char], index: &mut usize) -> Result<Expr, String> {
    if *index >= tokens.len() {
        return Err("Unexpected end of input".to_string());
    }

    match tokens[*index] {
        '0'..='9' => parse_number(tokens, index),
        '(' => {
            *index += 1;
            let expr = parse_expr(tokens, index, 0)?;

            if *index >= tokens.len() || tokens[*index] != ')' {
                return Err("Expected closing parenthesis".to_string());
            }
            *index += 1;
            Ok(expr)
        }
        _ => Err(format!("Unexpected character: {}", tokens[*index])),
    }
}

fn parse_number(tokens: &[char], index: &mut usize) -> Result<Expr, String> {
    let start = *index;
    while *index < tokens.len() && (tokens[*index].is_digit(10) || tokens[*index] == '.') {
        *index += 1;
    }
    let number: f64 = tokens[start..*index]
        .iter()
        .collect::<String>()
        .parse()
        .map_err(|e| format!("Failed to parse number: {}", e))?;
    Ok(Expr::Number(number))
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
                Operator::Potentiation => left_val.powf(right_val),
                Operator::CalculateRoot => left_val.powf(1.0 / right_val),
            };

            let step = format!("{} {} {} = {}", left_val, op.to_string(), right_val, result);
            left_steps.append(&mut right_steps);
            left_steps.push(step);
            (result, left_steps)
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_number() {
        let tokens: Vec<char> = "123.45".chars().collect();
        let mut index = 0;
        let result = parse_number(&tokens, &mut index).unwrap();
        assert_eq!(result, Expr::Number(123.45));
    }

    #[test]
    fn test_parse_expr_addition() {
        let expr = parse_expression("3+5").unwrap();
        if let Expr::Op(left, Operator::Add, right) = expr {
            assert_eq!(*left, Expr::Number(3.0));
            assert_eq!(*right, Expr::Number(5.0));
        } else {
            panic!("Failed to parse addition expression");
        }
    }

    #[test]
    fn test_parse_expr_subtraction() {
        let expr = parse_expression("10-4").unwrap();
        if let Expr::Op(left, Operator::Subtract, right) = expr {
            assert_eq!(*left, Expr::Number(10.0));
            assert_eq!(*right, Expr::Number(4.0));
        } else {
            panic!("Failed to parse subtraction expression");
        }
    }

    #[test]
    fn test_parse_expr_multiplication() {
        let expr = parse_expression("2*3").unwrap();
        if let Expr::Op(left, Operator::Multiply, right) = expr {
            assert_eq!(*left, Expr::Number(2.0));
            assert_eq!(*right, Expr::Number(3.0));
        } else {
            panic!("Failed to parse multiplication expression");
        }
    }

    #[test]
    fn test_parse_expr_division() {
        let expr = parse_expression("8/2").unwrap();
        if let Expr::Op(left, Operator::Divide, right) = expr {
            assert_eq!(*left, Expr::Number(8.0));
            assert_eq!(*right, Expr::Number(2.0));
        } else {
            panic!("Failed to parse division expression");
        }
    }

    #[test]
    fn test_parse_expr_potentiation() {
        let expr = parse_expression("2^3").unwrap();
        if let Expr::Op(left, Operator::Potentiation, right) = expr {
            assert_eq!(*left, Expr::Number(2.0));
            assert_eq!(*right, Expr::Number(3.0));
        } else {
            panic!("Failed to parse potentiation expression");
        }
    }

    #[test]
    fn test_parse_expr_calculate_root() {
        let expr = parse_expression("8r3").unwrap();
        if let Expr::Op(left, Operator::CalculateRoot, right) = expr {
            assert_eq!(*left, Expr::Number(8.0));
            assert_eq!(*right, Expr::Number(3.0));
        } else {
            panic!("Failed to parse root calculation expression");
        }
    }

    #[test]
    fn test_evaluate_addition() {
        let expr = parse_expression("3+5").unwrap();
        let (result, steps) = evaluate(&expr);
        assert_eq!(result, 8.0);
        assert_eq!(steps, vec!["3", "5", "3 + 5 = 8"]);
    }

    #[test]
    fn test_evaluate_subtraction() {
        let expr = parse_expression("10-4").unwrap();
        let (result, steps) = evaluate(&expr);
        assert_eq!(result, 6.0);
        assert_eq!(steps, vec!["10", "4", "10 - 4 = 6"]);
    }

    #[test]
    fn test_evaluate_multiplication() {
        let expr = parse_expression("2*3").unwrap();
        let (result, steps) = evaluate(&expr);
        assert_eq!(result, 6.0);
        assert_eq!(steps, vec!["2", "3", "2 * 3 = 6"]);
    }

    #[test]
    fn test_evaluate_division() {
        let expr = parse_expression("8/2").unwrap();
        let (result, steps) = evaluate(&expr);
        assert_eq!(result, 4.0);
        assert_eq!(steps, vec!["8", "2", "8 / 2 = 4"]);
    }

    #[test]
    fn test_evaluate_potentiation() {
        let expr = parse_expression("2^3").unwrap();
        let (result, steps) = evaluate(&expr);
        assert_eq!(result, 8.0);
        assert_eq!(steps, vec!["2", "3", "2 ^ 3 = 8"]);
    }

    #[test]
    fn test_evaluate_calculate_root() {
        let expr = parse_expression("8r3").unwrap();
        let (result, steps) = evaluate(&expr);
        assert_eq!(result, 2.0);
        assert_eq!(steps, vec!["8", "3", "8 r 3 = 2"]);
    }

    #[test]
    fn test_evaluate_complex_expression() {
        let expr = parse_expression("2+3*4-5/5").unwrap();
        let (result, steps) = evaluate(&expr);
        assert_eq!(result, 13.0);
        assert_eq!(
            steps,
            vec![
                "2",
                "3",
                "4",
                "3 * 4 = 12",
                "2 + 12 = 14",
                "5",
                "5",
                "5 / 5 = 1",
                "14 - 1 = 13"
            ]
        );
    }

    #[test]
    fn test_invalid_expression_unbalanced_parentheses() {
        let result = parse_expression("2*(3+4");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Expected closing parenthesis");
    }
}
