use axum::extract::{Json, Path, State};
use axum::routing::get;
use axum::{http::StatusCode, routing::post, Router};

use crate::errors::json_response_error::JsonResponseError;
use crate::extractors::require_authorization::RequireAuthorization;
use crate::state::application_state::ApplicationState;
use crate::v1::worlds::world_member_dto::WorldMemberDto;

use super::users_state::UsersState;

pub fn new() -> Router<ApplicationState> {
    Router::new().route("/@me/worlds-member", get(get_my_worlds))
}

async fn get_my_worlds(
    State(state): State<UsersState>,
    RequireAuthorization(user, _): RequireAuthorization<UsersState>,
) -> Result<Json<Vec<WorldMemberDto>>, JsonResponseError> {
    let worlds = state.world_member_service.get_user_world_member(user).await;

    println!("{:?}", worlds);

    match worlds {
        Ok(mut worlds) => Ok(Json(worlds.drain(..).map(|w| w.into()).collect())),
        Err(_) => Err(JsonResponseError::internal_server_error()),
    }
}
