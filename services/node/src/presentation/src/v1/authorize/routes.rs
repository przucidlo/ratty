use axum::{extract::Json, extract::State, http::StatusCode, routing::post, Router};
use serde::Deserialize;

use super::state::AuthorizationState;
use crate::state::ApplicationState;

#[derive(Deserialize)]
pub struct AuthorizeDto {
    username: String,
    password: String,
}

pub fn new() -> Router<ApplicationState> {
    Router::new().route("/", post(authorize))
}

async fn authorize(
    State(state): State<AuthorizationState>,
    Json(payload): Json<AuthorizeDto>,
) -> Result<String, StatusCode> {
    match state
        .authorization_service
        .authorize(&payload.username, &payload.password)
    {
        Ok(token) => Ok(token),
        Err(_) => Err(StatusCode::FORBIDDEN),
    }
}
