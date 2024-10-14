use crate::{
    structs::expression::{Expr, Operator},
    ClientError, Result,
};

pub fn parse_expression(input: &str) -> Result<Expr> {
    print!("{}", input);
    let mut index = 0;
    let tokens: Vec<char> = input.chars().filter(|c| !c.is_whitespace()).collect();
    parse_expr(&tokens, &mut index, 0)
}

fn parse_expr(tokens: &[char], index: &mut usize, min_precedence: u8) -> Result<Expr> {
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

fn parse_term(tokens: &[char], index: &mut usize) -> Result<Expr> {
    if *index >= tokens.len() {
        return Err(ClientError::UnexpectedEndOfInput);
    }

    match tokens[*index] {
        '0'..='9' | '-' => parse_number(tokens, index),
        '(' => {
            *index += 1;
            let expr = parse_expr(tokens, index, 0)?;

            if *index <= tokens.len() - 1 && tokens[*index - 1] == '^' {
                *index += 1;
            }

            if *index >= tokens.len() || tokens[*index] != ')' {
                return Err(ClientError::ExpectedClosingParenthesis);
            }
            *index += 1;
            Ok(expr)
        }
        _ => Err(ClientError::UnexpectedCharacter(tokens[*index].into())),
    }
}

fn parse_number(tokens: &[char], index: &mut usize) -> Result<Expr> {
    let mut start = *index;

    if *index > 0 && tokens[start] == '-' {
        *index += 1;
        start = *index;
    }

    while *index < tokens.len() && (tokens[*index].is_digit(10) || tokens[*index] == '.') {
        *index += 1;
    }

    let number_str: String = if *index > 0 && tokens[start - 1] == '-' {
        ["-", &tokens[start..*index].iter().collect::<String>()].concat()
    } else {
        tokens[start..*index].iter().collect()
    };

    let number: f64 = number_str.parse()?;

    Ok(Expr::Number(number))
}
