use application::authorization::authorization_service::Token;
use axum::{extract::Json, extract::State, http::StatusCode, routing::post, Router};
use serde::Deserialize;

use crate::state::application_state::ApplicationState;

use super::state::AuthorizationState;

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
) -> Result<Json<Token>, StatusCode> {
    match state
        .authorization_service
        .authorize(&payload.username, &payload.password)
        .await
    {
        Ok(token) => Ok(Json(token)),
        Err(_) => Err(StatusCode::FORBIDDEN),
    }
}
