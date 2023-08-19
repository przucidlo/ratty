use application::world::world_service::WorldService;
use axum::extract::FromRef;
use std::sync::Arc;

use crate::state::{
    application_state::ApplicationState,
    extractors_state::{ExtractorsState, WithExtractorsState},
};

#[derive(Clone)]
pub struct WorldsState {
    pub extractors_state: Arc<ExtractorsState>,
    pub world_service: Arc<WorldService>,
}

impl WithExtractorsState for WorldsState {
    fn extractors_state(&self) -> Arc<ExtractorsState> {
        self.extractors_state.clone()
    }
}

impl FromRef<ApplicationState> for WorldsState {
    fn from_ref(state: &ApplicationState) -> Self {
        state.worlds_state.clone()
    }
}
