use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};
use domain::user::user::User;

use crate::{
    errors::json_response_error::JsonResponseError,
    middleware::authentication_middleware::UserExtension,
    state::extractors_state::WithExtractorsState,
};

pub struct RequireAuthorization<S>(pub User, pub S);

#[async_trait]
impl<OuterState, InnerState> FromRequestParts<OuterState> for RequireAuthorization<InnerState>
where
    InnerState: Send + FromRef<OuterState> + WithExtractorsState,
    OuterState: Send + Sync,
{
    type Rejection = JsonResponseError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &OuterState,
    ) -> Result<Self, Self::Rejection> {
        if let Some(user_extension) = parts.extensions.get::<UserExtension>() {
            let inner_state = InnerState::from_ref(state);
            let extractors_state = inner_state.extractors_state();

            let user = extractors_state
                .user_service
                .get_user_by_id(&user_extension.user_id)
                .await;

            if let Ok(user) = user {
                Ok(Self(user, inner_state))
            } else {
                Err(JsonResponseError::unauthorized())
            }
        } else {
            // TODO: this should throw an alert
            Err(JsonResponseError::unauthorized())
        }
    }
}
