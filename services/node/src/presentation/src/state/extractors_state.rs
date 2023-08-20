use std::sync::Arc;

use application::user::user_service::UserService;

pub trait WithExtractorsState {
    fn extractors_state(&self) -> Arc<ExtractorsState>;
}

pub struct ExtractorsState {
    pub user_service: Arc<UserService>,
}
