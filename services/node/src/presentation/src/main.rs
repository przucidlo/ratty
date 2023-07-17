use application::application::Application;
use axum::Router;
use dotenvy::dotenv;
use state::ApplicationState;
use v1::authorize::state::AuthorizationState;

pub mod state;
pub mod v1;

use crate::v1::authorize::routes as AuthorizationRouter;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let application = Application::default();

    let state = ApplicationState {
        authorization_state: AuthorizationState {
            authorization_service: application.authorization_service.clone(),
        },
    };

    let app: Router = Router::new()
        .nest("/v1/authorize", AuthorizationRouter::new())
        .with_state(state);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
