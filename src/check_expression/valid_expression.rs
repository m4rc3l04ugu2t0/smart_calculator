use crate::{ClientError, Result};
use regex::Regex;

fn is_valid_expression(expression: &str) -> bool {
    // Verifica se há parênteses balanceados
    let mut balance = 0;
    for c in expression.chars() {
        if c == '(' {
            balance += 1;
        } else if c == ')' {
            balance -= 1;
        }
        if balance < 0 {
            return false; // Fechamento de parênteses sem correspondência
        }
    }
    if balance != 0 {
        return false; // Há parênteses sem correspondência
    }

    // Verifica operadores consecutivos, mas permite "^" como operador válido
    let re_invalid_ops = Regex::new(r"([+\*/]{2,}|[+\*/]$|^[\*/])").unwrap();
    if re_invalid_ops.is_match(expression) {
        return false; // Detecta operadores consecutivos ou incorretos
    }

    // Verifica se há operadores mal posicionados em relação aos parênteses
    let re_invalid_parens = Regex::new(r"\(\s*[+\*/]|\)\s*[+\*/]").unwrap();
    if re_invalid_parens.is_match(expression) {
        return false; // Detecta operadores mal posicionados em relação aos parênteses
    }

    true
}

pub fn format_expression(expression: &str) -> Result<String> {
    // Primeiro, valida a expressão
    if !is_valid_expression(expression) {
        return Err(ClientError::InvalidExpression); // Retorna um erro se a expressão for inválida
    }

    // Remove todos os espaços da expressão
    let re_spaces = Regex::new(r"\s+").unwrap();
    let mut expr = re_spaces.replace_all(expression, "").to_string();

    // Adiciona multiplicação implícita antes de parênteses, onde necessário
    let re_before_parens = Regex::new(r"(\d)\(").unwrap();
    expr = re_before_parens.replace_all(&expr, "$1*(").to_string();

    // Adiciona multiplicação implícita após parênteses, onde necessário
    let re_after_parens = Regex::new(r"\)(\d)").unwrap();
    expr = re_after_parens.replace_all(&expr, ")*$1").to_string();

    Ok(expr) // Retorna a expressão formatada se for válida
}
