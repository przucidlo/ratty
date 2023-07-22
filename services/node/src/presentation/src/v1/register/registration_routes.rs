use axum::{extract::Json, extract::State, http::StatusCode, routing::post, Router};
use serde::Deserialize;

use crate::state::ApplicationState;

use super::registration_state::RegistrationState;

#[derive(Deserialize)]
struct RegisterDto {
    username: String,
    password: String,
}

pub fn new() -> Router<ApplicationState> {
    Router::new().route("/", post(register))
}

async fn register(
    State(state): State<RegistrationState>,
    Json(payload): Json<RegisterDto>,
) -> StatusCode {
    let user = state
        .user_service
        .add_user(&payload.username, &payload.password)
        .await;

    match user {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
