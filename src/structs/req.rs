use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct CalculationResponse {
    pub result: f64,
    pub steps: Vec<String>,
    pub error: String,
}

#[derive(Deserialize)]
pub struct CalculationRequest {
    pub expression: String,
}
