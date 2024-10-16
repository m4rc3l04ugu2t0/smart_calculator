#[derive(Debug, PartialEq)]
pub enum Expr {
    Number(f64),
    Op(Box<Expr>, Operator, Box<Expr>),
}

#[derive(Debug, PartialEq)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Potentiation,
    CalculateRoot,
}
impl Operator {
    pub fn from_char(c: char) -> Option<Operator> {
        match c {
            '+' => Some(Operator::Add),
            '-' => Some(Operator::Subtract),
            '*' => Some(Operator::Multiply),
            '/' => Some(Operator::Divide),
            '^' => Some(Operator::Potentiation),
            'r' => Some(Operator::CalculateRoot),
            '(' => Some(Operator::Multiply),
            _ => None,
        }
    }

    pub fn to_string(&self) -> &str {
        match self {
            Operator::Add => "+",
            Operator::Subtract => "-",
            Operator::Multiply => "*",
            Operator::Divide => "/",
            Operator::Potentiation => "^",
            Operator::CalculateRoot => "r",
        }
    }

    pub fn precedence(&self) -> u8 {
        match self {
            Operator::Add | Operator::Subtract => 1,
            Operator::Multiply | Operator::Divide => 2,
            Operator::Potentiation | Operator::CalculateRoot => 3,
        }
    }
}
