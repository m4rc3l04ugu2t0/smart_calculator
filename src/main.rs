mod check_expression;
mod error;
mod evaluator;
mod parse_expression;
mod structs;
mod web;

use std::io::stdin;

use web::entry_point::calculate;

pub use self::error::{ClientError, Result};

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Invalid input");
    let app = calculate(input.trim().to_string());

    match app {
        Ok(sla) => {
            println!("{}", sla.result);
        }
        Err(e) => println!("{:?}", e),
    }
}
