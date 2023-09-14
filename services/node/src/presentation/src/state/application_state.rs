use crate::v1::{
    authorize::state::AuthorizationState, register::registration_state::RegistrationState,
    users::users_state::UsersState, worlds::worlds_state::WorldsState,
};

#[derive(Clone)]
pub struct ApplicationState {
    pub authorization_state: AuthorizationState,
    pub registration_state: RegistrationState,
    pub worlds_state: WorldsState,
    pub users_state: UsersState,
}

unsafe impl Sync for ApplicationState {}
unsafe impl Send for ApplicationState {}
