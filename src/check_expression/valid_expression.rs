use crate::{ClientError, Result};
use regex::Regex;

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

    let mut new_vec: String = new_vec.iter().collect();

    if new_vec[0..2].contains("-(") {
        new_vec = transform_negation(&new_vec);
    }

    let formatted_expression = format_expression(&new_vec);

    Ok(formatted_expression)
}
pub fn format_expression(expression: &str) -> String {
    // Remove todos os espaços da expressão
    let re_spaces = Regex::new(r"\s+").unwrap();
    let mut expr = re_spaces.replace_all(expression, "").to_string();

    // Adiciona multiplicação implícita antes de parênteses, onde necessário
    let re_before_parens = Regex::new(r"(\d)\(").unwrap();
    expr = re_before_parens.replace_all(&expr, "$1*(").to_string();

    // Adiciona multiplicação implícita após parênteses, onde necessário
    let re_after_parens = Regex::new(r"\)(\d)").unwrap();
    expr = re_after_parens.replace_all(&expr, ")*$1").to_string();

    // Altera "(+n" para "(n"
    let re_positive_in_parentheses = Regex::new(r"\(\+(\d)").unwrap();
    expr = re_positive_in_parentheses
        .replace_all(&expr, "($1")
        .to_string();

    expr // Retorna a expressão formatada se for válida
}

fn transform_negation(input: &str) -> String {
    // Expressão regular para capturar -(expressão dentro de parênteses)
    let re = Regex::new(r"-\(([^()]+)\)").unwrap();

    // Substituir todos os padrões encontrados por uma nova expressão com sinais invertidos
    let transformed = re
        .replace_all(input, |caps: &regex::Captures| {
            let inner_expr = &caps[1];

            // Inverter os sinais dentro da expressão
            let transformed = inner_expr
                .chars()
                .map(|c| match c {
                    '+' => '-',
                    '-' => '+',
                    _ => c,
                })
                .collect::<String>();

            transformed
        })
        .to_string();

    // Retornar a expressão transformada sem o parêntese
    if transformed.starts_with('+') {
        return transformed[1..].to_string();
    }
    format!("-{}", transformed)
}
