mod check_expression;
mod error;
mod evaluator;
mod structs;

use check_expression::valid_expression;
use evaluator::evaluate::evaluate;
use structs::req::CalculationResponse;

pub use self::error::{ClientError, Result};

use crate::structs::expression::{Expr, Operator};

pub fn parse_expression(input: &str) -> Result<Expr> {
    let mut index = 0;
    let tokens: Vec<char> = input.chars().filter(|c| !c.is_whitespace()).collect();
    println!("{:?}", tokens);
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

        let mut right = match op {
            Operator::Subtract => parse_negative_numeber(tokens, index),
            _ => parse_term(tokens, index),
        }?;

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

            right = match op {
                Operator::Subtract => parse_negative_numeber(tokens, index),
                _ => parse_term(tokens, index),
            }?;
        }

        left = Expr::Op(Box::new(left), op, Box::new(right));
    }
    println!("Expr finaly: {:?}", left);

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

fn parse_negative_numeber(tokens: &[char], index: &mut usize) -> Result<Expr> {
    let start = *index;

    while *index < tokens.len() && (tokens[*index].is_digit(10) || tokens[*index] == '.') {
        *index += 1;
    }

    let number_str: String = tokens[start..*index].iter().collect();

    let number: f64 = number_str.parse()?;

    Ok(Expr::Number(number))
}

fn main() {
    let input = "2r2";

    match valid_expression::format_expression(&input) {
        Ok(expression) => match parse_expression(&expression) {
            Ok(expr) => {
                let (result, steps) = evaluate(&expr);
                let response = CalculationResponse {
                    result,
                    steps,
                    status: ClientError::Successes,
                };
                println!("{:?}", response.steps);
            }
            Err(e) => {
                let response = CalculationResponse {
                    status: e,
                    result: 0.0,
                    steps: Vec::new(),
                };
                println!("{:?}", response.steps);
            }
        },
        Err(e) => {
            println!("{:?}", e)
        }
    }
}
