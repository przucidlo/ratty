use crate::v1::{
    authorize::state::AuthorizationState, register::registration_state::RegistrationState,
    worlds::worlds_state::WorldsState,
};

#[derive(Clone)]
pub struct ApplicationState {
    pub authorization_state: AuthorizationState,
    pub registration_state: RegistrationState,
    pub worlds_state: WorldsState,
}
