use crate::v1::{
    authorize::state::AuthorizationState, register::registration_state::RegistrationState,
};

#[derive(Clone)]
pub struct ApplicationState {
    pub authorization_state: AuthorizationState,
    pub registration_state: RegistrationState,
}
