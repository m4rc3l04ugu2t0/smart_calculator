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
            MathError::InvalidExpression => write!(f, "Invalid expression"),
            MathError::InvalidInput(input) => write!(f, "Invalid input provided: {}", input),
        }
    }
}

impl Error for MathError {}

pub fn valid_expression(expression: &str) -> Result<String, MathError> {
    let expression_chars: Vec<char> = expression.chars().collect();
    let mut parentheses_stack = Vec::new();
    let mut new_vec: Vec<char> = Vec::with_capacity(expression.len());
    let mut previous_char = ' ';

    for (index, &ch) in expression_chars.iter().enumerate() {
        match ch {
            '(' => {
                if index > 0
                    && (expression_chars[index - 1].is_digit(10)
                        || expression_chars[index - 1] == ')')
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
                if index < expression.len() - 1 && expression_chars[index + 1].is_digit(10) {
                    new_vec.push('*');
                }
            }
            '+' | '-' | '*' | '/' | '^' | 'r' => {
                if previous_char == ' ' || "+-*/".contains(previous_char) {
                    return Err(MathError::InvalidExpression);
                }
                new_vec.push(ch);
            }
            ch if ch.is_alphabetic() => {
                if previous_char.is_digit(10) || previous_char == ')' {
                    new_vec.push('*');
                }
                new_vec.push(ch);
            }
            '0'..='9' => {
                new_vec.push(ch);
            }
            _ => return Err(MathError::InvalidInput(expression.to_string())),
        }
        previous_char = ch;
    }

    if !parentheses_stack.is_empty() {
        return Err(MathError::InvalidExpression);
    }

    if "+-*/".contains(previous_char) {
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
    fn test_valid_expression() {
        let result = valid_expression("2(2+2)").expect("Failed to parse expression");
        assert_eq!("2*(2+2)".to_string(), result);
    }

    #[test]
    fn test_invalid_expression_unbalanced_parentheses() {
        let result = valid_expression("2(2+2").unwrap_err();
        assert_eq!(MathError::InvalidExpression.to_string(), result.to_string());
    }

    #[test]
    fn test_invalid_expression_operator_start() {
        let result = valid_expression("+2(2+2)").unwrap_err();
        assert_eq!(MathError::InvalidExpression.to_string(), result.to_string());
    }

    #[test]
    fn test_invalid_expression_operator_end() {
        let result = valid_expression("2(2+2)+").unwrap_err();
        assert_eq!(MathError::InvalidExpression.to_string(), result.to_string());
    }

    #[test]
    fn test_valid_expression_with_alphabetic_chars() {
        let result = valid_expression("2a+3b").expect("Failed to parse expression");
        assert_eq!("2*a+3*b".to_string(), result);
    }
}
