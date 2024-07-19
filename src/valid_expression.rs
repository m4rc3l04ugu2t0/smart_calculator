use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub enum MathError {
    InvalidExpression,
    InvalidInput(String),
}

impl Display for MathError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MathError::InvalidExpression => write!(f, "Expressão inválida"),
            MathError::InvalidInput(input) => write!(f, "Entrada inválida: {}", input),
        }
    }
}

impl Error for MathError {}

pub fn valid_expression(expression: &str) -> Result<(), MathError> {
    let mut parentheses_stack = Vec::new();
    let mut previous_char = ' ';

    for ch in expression.chars() {
        match ch {
            '(' | '[' | '{' => parentheses_stack.push(ch),
            ')' => {
                if parentheses_stack.pop() != Some('(') {
                    return Err(MathError::InvalidExpression);
                }
            }
            ']' => {
                if parentheses_stack.pop() != Some('[') {
                    return Err(MathError::InvalidExpression);
                }
            }
            '}' => {
                if parentheses_stack.pop() != Some('{') {
                    return Err(MathError::InvalidExpression);
                }
            }
            '+' | '-' | '*' | '/' | '^' | 'r' => {
                if previous_char == ' ' || "+-*/^r".contains(previous_char) {
                    return Err(MathError::InvalidExpression);
                }
            }
            ' ' => {}
            '0'..='9' => {}
            _ => return Err(MathError::InvalidInput(expression.to_string())),
        }
        previous_char = ch;
    }

    if !parentheses_stack.is_empty() {
        return Err(MathError::InvalidExpression);
    }

    if "+-*/^r".contains(previous_char) {
        return Err(MathError::InvalidExpression);
    }

    Ok(())
}
