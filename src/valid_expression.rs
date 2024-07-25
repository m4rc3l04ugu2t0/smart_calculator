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

// n() // 3k

pub fn valid_expression(expression: &str) -> Result<String, MathError> {
    let expression_chars: Vec<char> = expression.chars().collect();
    let mut parentheses_stack = Vec::new();
    let mut new_vec: Vec<char> = Vec::with_capacity(expression.len() - 1);
    let mut previus_char = ' ';

    for (index, &ch) in expression_chars.iter().enumerate() {
        match ch {
            '(' => {
                if index < expression.len() - 1
                    && expression_chars[index] == '('
                    && expression_chars[index - 1].is_digit(10)
                {
                    new_vec.push('*');
                }
                new_vec.push(ch);
                parentheses_stack.push(ch);
            }
            ')' => {
                if parentheses_stack.pop() != Some('(') {
                    return Err(MathError::InvalidExpression);
                }
                new_vec.push(ch);
            }
            '+' | '-' | '*' | '/' | '^' | 'r' => {
                if previus_char == ' ' || "+-*/".contains(previus_char) {
                    return Err(MathError::InvalidExpression);
                }

                new_vec.push(ch);
            }
            ch if ch.is_alphabetic() => {
                new_vec.push(ch);
            }
            '0'..='9' => {
                new_vec.push(ch);
            }
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

    let new_vec: String = new_vec.iter().collect();
    println!("{}", new_vec);

    Ok(new_vec)
}

#[cfg(test)]
mod test_valid_expression {
    use super::*;

    #[test]
    fn fn_tes_valid_expression() {
        let result = valid_expression("2(2+2)").expect("s,s");
        assert_eq!("2*(2+2)".to_string(), result);
    }
}
