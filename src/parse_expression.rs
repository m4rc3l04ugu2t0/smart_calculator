#[derive(Debug)]
pub enum Expr {
    Number(f64),
    Op(Box<Expr>, Operator, Box<Expr>), // Representa uma operação binaria, cada box é um numero.
}

#[derive(Debug)]
pub enum Operator {
    Add, // Declarando os types para as operações possiveis
    Subtract,
    Multiply,
    Divide,
    Potentiation,
    CalculateRoot,
}

impl Operator {
    // recebe um caracter e compara com o operador, o tipo que passa na verificação ira retorna
    // seu tipo correspondente ao operador
    fn from_char(c: char) -> Option<Operator> {
        match c {
            '+' => Some(Operator::Add),
            '-' => Some(Operator::Subtract),
            '*' => Some(Operator::Multiply),
            '/' => Some(Operator::Divide),
            '^' => Some(Operator::Potentiation),
            'r' => Some(Operator::CalculateRoot),
            _ => None,
        }
    }

    // compara o self que é o tipo implementado e compara com os tipos dos operadores implementados
    // passando na verificação é retornado o caraceter do operador
    fn to_string(&self) -> &str {
        match self {
            Operator::Add => "+",
            Operator::Subtract => "-",
            Operator::Multiply => "*",
            Operator::Divide => "/",
            Operator::Potentiation => "^",
            Operator::CalculateRoot => "√",
        }
    }

    fn order_precedence(&self) -> i8 {
        match self {
            Operator::Add | Operator::Subtract => 1,
            Operator::Divide | Operator::Multiply => 2,
            Operator::Potentiation => 3,
            _ => -1,
        }
    }
}

// função que vai pega o input do usuario e comverter para uma expressao valida com o auxilio
// das demais funções
pub fn parse_expression(input: &str) -> Result<Expr, String> {
    // tira todos os espaços em branco, itera sibre todas as caracteres em seguida filtrando cada
    // caracter e retornando elas sem espaço, depois tranformando em um vec de char
    let mut index = 0; // Inicializa o índice de parsing em 0.
    let tokens: Vec<char> = input.chars().filter(|c| !c.is_whitespace()).collect();
    parse_expr(&tokens, &mut index) //  Chama a função parse_expr para analisar a expressão e retorna o resultado.
}

fn parse_expr(tokens: &[char], index: &mut usize) -> Result<Expr, String> {
    let mut left = parse_term(tokens, index)?;

    while *index < tokens.len() {
        let op = match Operator::from_char(tokens[*index]) {
            Some(op) => op,
            None => break,
        };

        if op.order_precedence() > 

        *index += 1;
        let right = parse_term(tokens, index)?;
        left = Expr::Op(Box::new(left), op, Box::new(right));
    }

    Ok(left)
}

fn parse_term(tokens: &[char], index: &mut usize) -> Result<Expr, String> {
    if *index >= tokens.len() {
        // verifica se o indice é maior ou igual ao comprimento do vec de caracteres
        return Err("Unexpected end of input".to_string()); // case nao passe na validação retorna error
    }

    match tokens[*index] {
        '0'..='9' => parse_number(tokens, index), // verifica se é um numero velido na tabela unicode
        '(' => {
            *index += 1; // skip '('
            let expr = parse_expr(tokens, index)?;
            if *index >= tokens.len() || tokens[*index] != ')' {
                return Err("Expected closing parenthesis".to_string());
            }
            *index += 1; // skip ')'
            Ok(expr)
        }
        _ => Err(format!("Unexpected character: {}", tokens[*index])),
    }
}

fn parse_number(tokens: &[char], index: &mut usize) -> Result<Expr, String> {
    let start = *index;
    while *index < tokens.len() && (tokens[*index].is_digit(10) || tokens[*index] == '.') {
        *index += 1;
    }
    let number: f64 = tokens[start..*index]
        .iter()
        .collect::<String>()
        .parse()
        .map_err(|e| format!("Failed to parse number: {}", e))?;
    Ok(Expr::Number(number))
}

pub fn evaluate(expr: &Expr) -> (f64, Vec<String>) {
    match expr {
        Expr::Number(n) => (*n, vec![n.to_string()]),
        Expr::Op(left, op, right) => {
            let (left_val, mut left_steps) = evaluate(left);
            let (right_val, mut right_steps) = evaluate(right);

            let result = match op {
                Operator::Add => left_val + right_val,
                Operator::Subtract => left_val - right_val,
                Operator::Multiply => left_val * right_val,
                Operator::Divide => left_val / right_val,
                Operator::Potentiation => left_val.powf(right_val),
                Operator::CalculateRoot => left_val.powf(1.0 / right_val),
            };

            let step = format!("{} {} {} = {}", left_val, op.to_string(), right_val, result);
            left_steps.append(&mut right_steps);
            left_steps.push(step);
            (result, left_steps)
        }
    }
}
