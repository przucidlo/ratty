use axum::{extract::State, routing::get, Router};

#[derive(Clone)]
pub struct RegisterState {
    test: isize,
}

pub fn new() -> Router<RegisterState> {
    let state = RegisterState { test: 666 };

    Router::new().route("/", get(test_get)).with_state(state)
}

async fn test_get(State(state): State<RegisterState>) -> String {
    state.test.to_string()
}
