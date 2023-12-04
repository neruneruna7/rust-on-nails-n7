use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use std::fmt;
use db::{TokioPostgressError, PoolError};

#[derive(Debug)]
pub enum CustomError {
    FaultySetup(String),
    Database(String),
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CustomError::FaultySetup(ref cause) => write!(f, "Setup Error: {}", cause),
            CustomError::Database(ref cause) => write!(f, "Database error: {}", cause),
        }
    }
}

impl IntoResponse for CustomError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            CustomError::Database(message) => (StatusCode::UNPROCESSABLE_ENTITY, message),
            CustomError::FaultySetup(message) => (StatusCode::UNPROCESSABLE_ENTITY, message),
        };

        format!("status = {}, message = {}", status, error_message).into_response()
    }
}

impl From<axum::http::uri::InvalidUri> for CustomError {
    fn from(err: axum::http::uri::InvalidUri) -> Self {
        CustomError::FaultySetup(err.to_string())
    }
}

impl From<TokioPostgressError> for CustomError {
    fn from(err: TokioPostgressError) -> Self {
        CustomError::Database(err.to_string())
    }
}

impl From<PoolError> for CustomError {
    fn from(err: PoolError) -> Self {
        CustomError::Database(err.to_string())
    }
}