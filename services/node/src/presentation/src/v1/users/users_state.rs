use application::{
    user::user_service::UserService, world::world_member_service::WorldMemberService,
};
use axum::extract::FromRef;
use std::sync::Arc;

use crate::state::{
    application_state::ApplicationState,
    extractors_state::{ExtractorsState, WithExtractorsState},
};

#[derive(Clone)]
pub struct UsersState {
    pub extractors_state: Arc<ExtractorsState>,
    pub user_service: Arc<UserService>,
    pub world_member_service: Arc<WorldMemberService>,
}

impl WithExtractorsState for UsersState {
    fn extractors_state(&self) -> Arc<ExtractorsState> {
        self.extractors_state.clone()
    }
}

impl FromRef<ApplicationState> for UsersState {
    fn from_ref(state: &ApplicationState) -> Self {
        state.users_state.clone()
    }
}
