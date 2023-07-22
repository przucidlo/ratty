use crate::state::ApplicationState;
use application::user::user_service::UserService;
use axum::extract::FromRef;
use std::sync::Arc;

#[derive(Clone)]
pub struct RegistrationState {
    pub user_service: Arc<UserService>,
}

impl FromRef<ApplicationState> for RegistrationState {
    fn from_ref(state: &ApplicationState) -> Self {
        state.registration_state.clone()
    }
}
