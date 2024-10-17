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
    println!("{}", input);
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

            if tokens[*index] == '^' || tokens[*index] == 'r' {
                *index += 1;
                let number = parse_number(tokens, index)?;

                right = Expr::Op(Box::new(right), next_op, Box::new(number));
                break;
            }

            if tokens[*index] == '*' {
                *index -= 1;
            }

            right = match next_op {
                Operator::Subtract => parse_negative_numeber(tokens, index),
                _ => parse_expr(tokens, index, next_op.precedence()),
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
    let test_expressions = vec![
        "-1+(4+5)^2-4",  // Expressão com número negativo, adição, potência e subtração
        "2(3+4)",        // Multiplicação implícita
        "(2+2)r3",       // Raiz cúbica
        "(3^2)r5",       // Raiz de quinto grau após potência
        "5+3*2-1",       // Expressão simples com operadores básicos
        "(1+1)/2",       // Divisão por zero
        "((2+3)*(4-1))", // Parênteses aninhados
        "-2-2",          // Subtração com números negativos
        "2+(-3*4)",      // Operação com número negativo dentro de parênteses
        "(4+5)^2/2",     // Potência seguida de divisão por zero
        "2^(3+1)",       // Potência com expressão dentro
        "27r4",
    ];

    for input in test_expressions {
        println!("Testando a expressão: {}", input);

        match valid_expression::valid_expression(&input) {
            Ok(expression) => match parse_expression(&expression) {
                Ok(expr) => {
                    let (result, steps) = evaluate(&expr);
                    let response = CalculationResponse {
                        result,
                        steps,
                        status: ClientError::Successes,
                    };
                    println!(
                        "Resultado: {}, Passos: {:?}",
                        response.result, response.steps
                    );
                }
                Err(e) => {
                    let response = CalculationResponse {
                        status: e,
                        result: 0.0,
                        steps: Vec::new(),
                    };
                    println!("Erro ao analisar a expressão: {:?}", response.status);
                }
            },
            Err(e) => {
                println!("Erro ao validar a expressão: {:?}", e);
            }
        }

        println!("-------------------------------");
    }
}
