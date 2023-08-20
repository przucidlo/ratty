use std::str::FromStr;

use axum::extract::{Json, State};
use axum::{http::StatusCode, routing::post, Router};
use domain::world::world::WorldKind;

use crate::extractors::require_authorization::RequireAuthorization;
use crate::extractors::validate_json::ValidateJson;
use crate::state::application_state::ApplicationState;

use super::world_dto::WorldDto;
use super::worlds_state::WorldsState;

pub fn new() -> Router<ApplicationState> {
    Router::new().route("/", post(create_world))
}

async fn create_world(
    State(state): State<WorldsState>,
    RequireAuthorization(user, _): RequireAuthorization<WorldsState>,
    ValidateJson(payload): ValidateJson<WorldDto>,
) -> Result<Json<WorldDto>, StatusCode> {
    let description = payload.description.unwrap_or("".to_owned());

    let world = state
        .world_service
        .create_world(
            &payload.name,
            &description,
            WorldKind::from_str(&payload.kind).unwrap(),
            user,
        )
        .await;

    match world {
        Ok(world) => Ok(Json(world.into())),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
