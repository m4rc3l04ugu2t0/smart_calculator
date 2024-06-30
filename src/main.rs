use meval::eval_str;
use std::io::{self, Write};

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

        match calcular_e_exibir_passos(input) {
            Ok(resultado) => println!("Resultado: {}", resultado),
            Err(e) => println!("Erro: {}", e),
        }
    }
}

fn calcular_e_exibir_passos(expressao: &str) -> Result<f64, String> {
    // Passo 1: Mostrar a expressão original
    println!("Expressão original: {}", expressao);

    // Passo 2: Simplificação básica (Remover espaços)
    let expressao_simplificada = expressao.replace(" ", "");
    println!("Expressão simplificada: {}", expressao_simplificada);

    // Passo 3: Avaliar a expressão
    match eval_str(&expressao_simplificada) {
        Ok(resultado) => {
            println!("Avaliação da expressão: {}", resultado);
            Ok(resultado)
        }
        Err(e) => Err(format!("Erro na avaliação: {}", e)),
    }
}
