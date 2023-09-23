use std::sync::Arc;

use application::{application::Application, user::user_service};
use axum::Router;
use dotenvy::dotenv;
use middleware::authentication_middleware::authentication_middleware;
use state::{application_state::ApplicationState, extractors_state::ExtractorsState};
use tower_http::cors::CorsLayer;
use v1::{
    authorize::state::AuthorizationState,
    register::{registration_routes, registration_state::RegistrationState},
    users::{users_routes, users_state::UsersState},
    worlds::{worlds_routes, worlds_state::WorldsState},
};

use axum::middleware as axum_middleware;

pub mod errors;
pub mod extractors;
pub mod middleware;
pub mod state;
pub mod v1;
pub mod validation;

use crate::v1::authorize::routes as AuthorizationRouter;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let application = Application::new().await;

    let extractors_state = Arc::new(ExtractorsState {
        user_service: application.user_service.clone(),
    });

    let state = ApplicationState {
        authorization_state: AuthorizationState {
            authorization_service: application.authorization_service.clone(),
        },
        registration_state: RegistrationState {
            user_service: application.user_service.clone(),
        },
        worlds_state: WorldsState {
            extractors_state: extractors_state.clone(),
            world_service: application.world_service.clone(),
        },
        users_state: UsersState {
            extractors_state: extractors_state.clone(),
            user_service: application.user_service.clone(),
            world_member_service: application.world_member_service.clone(),
        },
    };

    let app: Router = Router::new()
        .nest("/v1/authorize", AuthorizationRouter::new())
        .nest("/v1/register", registration_routes::new())
        .nest("/v1/worlds", worlds_routes::new())
        .nest("/v1/users", users_routes::new())
        .layer(axum_middleware::from_fn(authentication_middleware))
        .layer(CorsLayer::permissive())
        .with_state(state);

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
