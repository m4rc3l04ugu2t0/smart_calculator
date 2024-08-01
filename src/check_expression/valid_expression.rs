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

    Ok(new_vec)
}
#[tokio::test]
async fn test_valid_expression() {
    // Teste com uma expressão válida
    let expr = "3 + (2 - 1) * 5";
    let result = valid_expression(expr).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "3+(2-1)*5");

    // Teste com uma expressão inválida (parênteses desbalanceados)
    let expr = "3 + (2 - 1 * 5";
    let result = valid_expression(expr).await;
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), ClientError::InvalidExpression);

    // Teste com uma expressão inválida (caracteres inválidos)
    let expr = "3 + 2a - 1";
    let result = valid_expression(expr).await;
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        ClientError::InvalidInput("3 + 2a - 1".to_string())
    );

    // Teste com uma expressão com caracteres alfabéticos
    let expr = "3a + 2b";
    let result = valid_expression(expr).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "3*a+2*b");

    // Teste com expressão que termina com operador inválido
    let expr = "3 + 2 -";
    let result = valid_expression(expr).await;
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), ClientError::InvalidExpression);

    // Teste com expressão contendo ponto decimal
    let expr = "3.5 + 2.1";
    let result = valid_expression(expr).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "3.5+2.1");
}
