use application::user::user_errors::AddUserError;
use axum::{extract::State, http::StatusCode, routing::post, Router};
use garde::Validate;
use serde::Deserialize;

use crate::{
    extractors::validated_json::ValidatedJson, state::application_state::ApplicationState,
};

use super::registration_state::RegistrationState;

#[derive(Deserialize, Validate)]
struct RegisterDto {
    #[garde(length(min = 3))]
    username: String,

    #[garde(length(min = 3))]
    password: String,
}

pub fn new() -> Router<ApplicationState> {
    Router::new().route("/", post(register))
}

#[axum_macros::debug_handler]
async fn register(
    State(state): State<RegistrationState>,
    ValidatedJson(payload): ValidatedJson<RegisterDto>,
) -> StatusCode {
    let user = state
        .user_service
        .add_user(&payload.username, &payload.password)
        .await;

    match user {
        Ok(_) => StatusCode::CREATED,
        Err(err) => match err {
            AddUserError::UserAlreadyExistsError => StatusCode::CONFLICT,
            AddUserError::UserRepositoryFailureError => StatusCode::INTERNAL_SERVER_ERROR,
        },
    }
}
