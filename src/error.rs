use std::num::ParseFloatError;

use serde::Serialize;

pub type Result<T> = core::result::Result<T, ClientError>;

#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum ClientError {
    InvalidExpression,
    InvalidInput(String),
    ExpectedClosingParenthesis,
    UnexpectedEndOfInput,
    UnexpectedCharacter(String),
    FailedToParseNumber(String),
    Successes,
}
impl ClientError {
    pub fn expression_status_error(&self) -> ClientError {
        match self {
            Self::InvalidExpression => ClientError::InvalidExpression,
            Self::InvalidInput(input) => ClientError::InvalidInput(input.to_string()),
            Self::ExpectedClosingParenthesis => ClientError::ExpectedClosingParenthesis,
            Self::UnexpectedCharacter(c) => ClientError::UnexpectedCharacter(c.into()),
            Self::UnexpectedEndOfInput => ClientError::UnexpectedEndOfInput,
            Self::FailedToParseNumber(e) => ClientError::FailedToParseNumber(e.into()),
            Self::Successes => ClientError::Successes,
        }
    }
}

impl From<ParseFloatError> for ClientError {
    fn from(value: ParseFloatError) -> Self {
        ClientError::FailedToParseNumber(value.to_string())
    }
}
