use std::{error::Error, fmt::Display};

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Serialize)]
pub struct JsonResponseError {
    status: u16,
    message: Value,
}

impl Error for JsonResponseError {}

impl Display for JsonResponseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message.to_string())
    }
}

impl IntoResponse for JsonResponseError {
    fn into_response(self) -> Response {
        let status_code =
            StatusCode::from_u16(self.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        let mut response = Json(self).into_response();

        *response.status_mut() = status_code;

        response
    }
}

impl From<Response> for JsonResponseError {
    fn from(value: Response) -> Self {
        Self {
            status: value.status().as_u16(),
            message: Value::String(
                value
                    .status()
                    .canonical_reason()
                    .unwrap_or("Internal Server Error")
                    .to_string(),
            ),
        }
    }
}

impl JsonResponseError {
    pub fn from_str(status: u16, message: &str) -> Self {
        Self {
            status,
            message: Value::String(message.to_string()),
        }
    }

    pub fn from_validation_errors(errors: Value) -> Self {
        Self {
            status: 400,
            message: errors,
        }
    }

    pub fn unauthorized() -> Self {
        let status = StatusCode::UNAUTHORIZED;

        Self {
            status: status.as_u16(),
            message: Value::String(
                status
                    .canonical_reason()
                    .unwrap_or("Unauthorized")
                    .to_string(),
            ),
        }
    }

    pub fn internal_server_error() -> Self {
        Self {
            status: 500,
            message: Value::String("Internal Server Error".to_string()),
        }
    }

    pub fn hide_message(&mut self) {
        let status_code =
            StatusCode::from_u16(self.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

        self.message = Value::String(status_code.canonical_reason().unwrap().to_string());
    }
}
