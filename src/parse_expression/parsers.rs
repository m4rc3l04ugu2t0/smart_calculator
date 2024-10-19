use crate::{
    structs::expression::{Expr, Operator},
    ClientError, Result,
};

pub fn parse_expression(input: &str) -> Result<Expr> {
    let mut index = 0;
    let tokens: Vec<char> = input.chars().filter(|c| !c.is_whitespace()).collect();
    parse_expr(&tokens, &mut index, 0)
}

fn parse_expr(tokens: &[char], index: &mut usize, min_precedence: u8) -> Result<Expr> {
    let mut left = parse_term(tokens, index)?;

    while *index < tokens.len() {
        let mut op = match Operator::from_char(tokens[*index]) {
            Some(op) => op,
            None => break,
        };

        if op.precedence() < min_precedence {
            break;
        }

        *index += 1;

        let mut right = match op {
            Operator::Subtract => {
                if tokens[*index] == '(' {
                    parse_term(tokens, index)
                } else {
                    op = Operator::Add;
                    parse_expr(tokens, index, op.precedence())
                }
            }
            _ => parse_term(tokens, index),
        }?;

        while *index < tokens.len() {
            let mut next_op = match Operator::from_char(tokens[*index]) {
                Some(next_op) => next_op,
                None => break,
            };

            if next_op.precedence() <= op.precedence() {
                break;
            }

            if tokens[*index] == '^' || tokens[*index] == 'r' {
                *index += 1;
                let number = parse_number(tokens, index)?;

                right = Expr::Op(Box::new(right), next_op, Box::new(number));
                break;
            }

            if tokens[*index] == '*' || tokens[*index] == '/' {
                *index -= 1;
            }

            right = match next_op {
                Operator::Subtract => {
                    if tokens[*index] == '(' {
                        parse_term(tokens, index)
                    } else {
                        next_op = Operator::Add;
                        parse_expr(tokens, index, next_op.precedence())
                    }
                }
                _ => parse_expr(tokens, index, next_op.precedence()),
            }?;
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
        '0'..='9' | '-' | '+' => parse_number(tokens, index),
        '(' => {
            *index += 1;
            let expr = parse_expr(tokens, index, 0)?;
            if *index >= tokens.len() || tokens[*index] != ')' {
                return Err(ClientError::ExpectedClosingParenthesis);
            }
            *index += 1;
            Ok(expr)
        }
        _ => Err(ClientError::UnexpectedCharacter(format!(
            "Unexpected character '{}'",
            tokens[*index]
        ))),
    }
}

fn parse_number(tokens: &[char], index: &mut usize) -> Result<Expr> {
    let mut start = *index;

    if *index < tokens.len() && tokens[start] == '-' {
        *index += 1;
        start = *index;
    }

    while *index < tokens.len() && (tokens[*index].is_digit(10) || tokens[*index] == '.') {
        *index += 1;
    }

    let number_str: String = if start > 0 && tokens[start - 1] == '-' {
        ["-", &tokens[start..*index].iter().collect::<String>()].concat()
    } else {
        tokens[start..*index].iter().collect()
    };

    let number: f64 = number_str.parse()?;

    Ok(Expr::Number(number))
}
