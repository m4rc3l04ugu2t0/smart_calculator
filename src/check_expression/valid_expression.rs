use crate::{ClientError, Result};

pub async fn valid_expression(expression: &str) -> Result<String> {
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
                    return Err(ClientError::InvalidExpression);
                }
                new_vec.push(ch);
                if index < expression.len() - 1 && expression_chars[index + 1].is_digit(10) {
                    new_vec.push('*');
                }
            }
            '+' | '-' | '*' | '/' | '^' | 'r' => {
                if (previous_char == ' ' || "+-*/".contains(previous_char))
                    && index > 0
                    && ch != '-'
                {
                    return Err(ClientError::InvalidExpression);
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
            '.' => {
                new_vec.push(ch);
            }
            _ => return Err(ClientError::InvalidInput(expression.to_string())),
        }
        previous_char = ch;
    }

    if !parentheses_stack.is_empty() {
        return Err(ClientError::InvalidExpression);
    }

    if "+-*/".contains(previous_char) {
        return Err(ClientError::InvalidExpression);
    }

    let new_vec: String = new_vec.iter().collect();
    println!("new vec: {}", new_vec);

    Ok(new_vec)
}
