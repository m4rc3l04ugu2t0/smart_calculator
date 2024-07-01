use std::io::{self, Write};

mod parse_expression;
mod valid_expression;

use parse_expression::{evaluate, parse_expression};
use valid_expression::valid_expression;

fn main() {
    println!("Calculadora Inteligente");
    loop {
        print!("Digite a expressão (ou 'sair' para terminar): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Falha ao ler linha");
        let input = input.trim();

        if input.to_lowercase() == "sair" {
            break;
        }

        match calculate_and_display_steps(input) {
            Ok(resultado) => match parse_expression(input) {
                Ok(expr) => {
                    let (result, steps) = evaluate(&expr);
                    println!("Result: {}", result);
                    println!("Steps:");
                    for step in steps {
                        println!("{}", step);
                    }
                }
                Err(e) => println!("Error: {}", e),
            },
            Err(e) => println!("Erro: {}", e),
        }
    }
}

fn calculate_and_display_steps(expressao: &str) -> Result<bool, String> {
    // Passo 1: Mostrar a expressão original
    println!("Expressão original: {}", expressao);

    // Passo 2: Simplificação básica (Remover espaços)
    let expressao_simplificada = expressao.replace(" ", "");
    println!("Expressão simplificada: {}", expressao_simplificada);

    // Passo 3: Avaliar a expressão
    match valid_expression(&expressao_simplificada) {
        Ok(resultado) => {
            println!("Avaliação da expressão: {}", resultado);
            Ok(resultado)
        }
        Err(e) => Err(format!("Erro na avaliação: {}", e)),
    }
}
