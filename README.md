# Smart Calculator

Este projeto é uma calculadora inteligente desenvolvida em Rust que mostra passo a passo o processo para chegar à resposta. Utiliza o framework Axum para a parte web.

## Funcionalidades

- **Validação de Expressões**: Verifica se a expressão fornecida é válida.
- **Parseamento de Expressões**: Analisa a expressão para convertê-la em uma estrutura compreensível pelo programa.
- **Avaliação de Expressões**: Calcula o resultado da expressão e mostra os passos detalhados.
- **API REST**: Endpoint para calcular a expressão via requisição HTTP.

## Requisitos

- Rust
- Cargo
- Tokio
- Axum
- Serde

## Instalação

1. Clone o repositório:

    ```bash
    git clone https://github.com/seu-usuario/smart-calculator.git
    cd smart-calculator
    ```

2. Compile o projeto:

    ```bash
    cargo build --release
    ```

## Uso

1. Inicie o servidor:

    ```bash
    cargo run --release
    ```

2. Envie uma requisição POST para o endpoint `/calculate` com uma expressão no corpo da requisição. Por exemplo:

    ```json
    {
        "expression": "3 + 4 * 2 / ( 1 - 5 ) ^ 2 ^ 3"
    }
    ```

3. O servidor retornará o resultado e os passos para calcular a expressão.

## Estrutura do Projeto

- **check_expression**: Módulo responsável pela validação das expressões.
- **error**: Módulo que define os tipos de erro utilizados no projeto.
- **evaluator**: Módulo que avalia as expressões e calcula o resultado.
- **parse_expression**: Módulo que analisa as expressões e converte para a estrutura interna.
- **structs**: Módulo que define as estruturas utilizadas no projeto.
- **web**: Módulo que lida com as requisições HTTP.

## Exemplo de Requisição

Envie uma requisição POST para o endpoint `/calculate` com o seguinte corpo JSON:

```json
{
    "expression": "3 + 4 * 2 / ( 1 - 5 ) ^ 2 ^ 3"
}
```

Resposta:

```json
{
    "result": 3.0001220703125,
    "steps": [
        "4 * 2 = 8",
        "1 - 5 = -4",
        "-4 ^ 2 = 16",
        "8 / 16 = 0.5",
        "3 + 0.5 = 3.5"
    ],
    "error": ""
}
```