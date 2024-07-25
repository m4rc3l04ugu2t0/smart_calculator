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
        let input = input.trim().replace(" ", "");

        if input.to_lowercase() == "sair" {
            break;
        }

        match valid_expression(&input) {
            Ok(expr) => match parse_expression(&expr) {
                Ok(expr) => {
                    println!("{:?}", expr);
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
