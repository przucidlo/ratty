use crate::v1::authorize::state::AuthorizationState;

#[derive(Clone)]
pub struct ApplicationState {
    pub authorization_state: AuthorizationState,
}
