use axum::{http::StatusCode, response::IntoResponse};
use serde::Serialize;

pub type Result<T> = core::result::Result<T, ClientError>;

#[derive(Debug, Clone, Serialize)]
pub enum ClientError {
    InvalidExpression,
    InvalidInput(String),
}

impl IntoResponse for ClientError {
    fn into_response(self) -> axum::response::Response {
        println!("->> {:<12} - {self:?}", "INTO_RES");
        let mut response =
            (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_CLIENT_ERROR").into_response();

        response.extensions_mut().insert(self);

        response
    }
}

impl ClientError {
    pub fn expression_status_error(&self) -> ClientError {
        match self {
            Self::InvalidExpression => ClientError::InvalidExpression,
            Self::InvalidInput(input) => ClientError::InvalidInput(input.to_string()),
        }
    }
}
