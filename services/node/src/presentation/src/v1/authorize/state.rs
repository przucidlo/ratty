use crate::state::ApplicationState;
use application::authorization::authorization_service::AuthorizationService;
use axum::extract::FromRef;
use std::sync::Arc;

#[derive(Clone)]
pub struct AuthorizationState {
    pub authorization_service: Arc<AuthorizationService>,
}

impl FromRef<ApplicationState> for AuthorizationState {
    fn from_ref(state: &ApplicationState) -> Self {
        state.authorization_state.clone()
    }
}
