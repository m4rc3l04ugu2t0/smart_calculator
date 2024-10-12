use crate::ClientError;

#[warn(dead_code)]
pub struct CalculationResponse {
    pub result: f64,
    pub steps: Vec<String>,
    pub status: ClientError,
}

pub struct CalculationRequest {
    pub expression: String,
}
