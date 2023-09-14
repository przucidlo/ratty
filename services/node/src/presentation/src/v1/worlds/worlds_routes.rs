use std::str::FromStr;

use axum::extract::{Json, Path, State};
use axum::routing::{get, put};
use axum::{http::StatusCode, routing::post, Router};
use domain::world::world_error::JoinWorldError;
use domain::world::world_kind::WorldKind;

use crate::errors::json_response_error::JsonResponseError;
use crate::extractors::require_authorization::RequireAuthorization;
use crate::extractors::validate_json::ValidateJson;
use crate::state::application_state::ApplicationState;

use super::world_dto::WorldDto;
use super::worlds_state::WorldsState;

pub fn new() -> Router<ApplicationState> {
    Router::new()
        .route("/", post(create_world))
        .route("/", get(get_worlds))
        .route("/:id/members", put(join_world))
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

async fn join_world(
    State(state): State<WorldsState>,
    Path(id): Path<u64>,
    RequireAuthorization(user, _): RequireAuthorization<WorldsState>,
) -> Result<(), JsonResponseError> {
    let world_member = state.world_service.join_world(user, id).await;

    match world_member {
        Ok(_) => Ok(()),
        Err(e) => match e {
            JoinWorldError::WorldNotFoundError => {
                Err(JsonResponseError::not_found_error(Some("World not found")))
            }
            JoinWorldError::WorldAlreadyJoinedError => Err(JsonResponseError::forbidden(Some(
                "Could not join same world twice.",
            ))),
            JoinWorldError::WorldJoinFailedError => Err(JsonResponseError::internal_server_error()),
            JoinWorldError::WorldRequiresInvitationError => Err(JsonResponseError::forbidden(
                Some("World requires an invitation to become a member"),
            )),
        },
    }
}

async fn get_worlds(
    State(state): State<WorldsState>,
    RequireAuthorization(_, _): RequireAuthorization<WorldsState>,
) -> Result<Json<Vec<WorldDto>>, JsonResponseError> {
    let worlds = state.world_service.get_public_worlds().await;

    match worlds {
        Ok(mut worlds) => Ok(Json(worlds.drain(..).map(|w| w.into()).collect())),
        Err(_) => Err(JsonResponseError::internal_server_error()),
    }
}
