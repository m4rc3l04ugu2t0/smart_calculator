use crate::{
    check_expression::valid_expression::valid_expression, evaluator::evaluate::evaluate,
    parse_expression::parsers::parse_expression, structs::req::CalculationResponse, ClientError,
};

pub fn calculate(input: String) -> Result<CalculationResponse, ClientError> {
    match valid_expression(&input) {
        Ok(expression) => match parse_expression(&expression) {
            Ok(expr) => {
                println!("{}", expression);
                let (result, steps) = evaluate(&expr);
                let response = CalculationResponse {
                    result,
                    steps,
                    status: ClientError::Successes,
                };
                Ok(response)
            }
            Err(e) => {
                let response = CalculationResponse {
                    status: e,
                    result: 0.0,
                    steps: Vec::new(),
                };
                Ok(response)
            }
        },
        Err(e) => Err(e),
    }
}
