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
            MathError::InvalidExpression => write!(f, "Attempted to divide by zero"),
            MathError::InvalidInput(input) => write!(f, "Invalid input provided: {}", input),
        }
    }
}

impl Error for MathError {}

pub fn valid_expression(expression: &str) -> Result<bool, MathError> {
    let mut parentheses_stack = Vec::new();
    let mut previus_char = ' ';

    for ch in expression.chars() {
        match ch {
            '(' => {
                parentheses_stack.push(ch);
            }
            ')' => {
                if parentheses_stack.pop() != Some('(') {
                    return Err(MathError::InvalidExpression);
                }
            }
            '+' | '-' | '*' | '/' | '^' | 'r' => {
                if previus_char == ' ' || "+-*/".contains(previus_char) {
                    return Err(MathError::InvalidExpression);
                }
            }
            ' ' => {}
            '0'..='9' => {}
            _ => return Err(MathError::InvalidInput(expression.to_string())),
        }
        previus_char = ch;
    }

    if !parentheses_stack.is_empty() {
        return Err(MathError::InvalidExpression);
    }

    if "+-*/".contains(previus_char) {
        return Err(MathError::InvalidExpression);
    }

    Ok(true)
}
