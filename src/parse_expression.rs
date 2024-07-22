#[derive(Debug)]
pub enum Expr {
    Number(f64),
    Op(Box<Expr>, Operator, Box<Expr>), // Representa uma operação binária, cada box é um número.
}

#[derive(Debug)]
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

        if min_precedence == 2 && min_precedence > 1 {
            println!("oi");
        } else {
            *index += 1;
        }

        let mut right = parse_term(tokens, index)?;

        while *index < tokens.len() {
            if *index > 0 && tokens[*index] == ')' && tokens[*index + 1] == '^' {
                let result = Ok(Expr::Op(
                    Box::new(Expr::Op(Box::new(left), op, Box::new(right))),
                    Operator::Potentiation,
                    Box::new(Expr::Number(
                        tokens[*index + 2].to_digit(10).expect("Error").into(),
                    )),
                ));

                println!("{:?}", result);
                return result;
            }
            if *index > 0 && !tokens[*index - 1].is_digit(10) && tokens[*index] == '(' {
                break;
            }

            let next_op = match Operator::from_char(tokens[*index]) {
                Some(next_op) => next_op,
                None => break,
            };

            if next_op.precedence() <= op.precedence() {
                break;
            }

            if next_op.precedence() == 2 || tokens[*index] == '*' {
                *index -= 1;
                right = parse_expr(tokens, index, next_op.precedence())?;
            } else {
                *index += 1;
                right = parse_expr(tokens, index, next_op.precedence())?;
            }
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
            if *index > 0 && tokens[*index] == '^' {
                *index = tokens.len();
            }
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
    println!("{:?}", expr);
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
