use serde::{Deserialize, Serialize};

use crate::ClientError;

#[derive(Serialize)]
pub struct CalculationResponse {
    pub result: f64,
    pub steps: Vec<String>,
    pub error: ClientError,
}

#[derive(Deserialize)]
pub struct CalculationRequest {
    pub expression: String,
}
