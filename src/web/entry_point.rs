use axum::{response::IntoResponse, Json};

use crate::{
    check_expression::valid_expression::valid_expression,
    evaluator::evaluate::evaluate,
    parse_expression::parsers::parse_expression,
    structs::req::{CalculationRequest, CalculationResponse},
    ClientError,
};

pub async fn calculate(Json(payload): Json<CalculationRequest>) -> impl IntoResponse {
    match valid_expression(&payload.expression).await {
        Ok(expression) => match parse_expression(&expression) {
            Ok(expr) => {
                println!("{}", expression);
                let (result, steps) = evaluate(&expr);
                let response = CalculationResponse {
                    result,
                    steps,
                    status: ClientError::Successes,
                };
                Json(response).into_response()
            }
            Err(e) => {
                let response = CalculationResponse {
                    status: e,
                    result: 0.0,
                    steps: Vec::new(),
                };
                Json(response).into_response()
            }
        },
        Err(e) => e.into_response(),
    }
}
