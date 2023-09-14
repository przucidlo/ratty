use std::sync::Arc;

use infrastructure::{
    config::config_factory::ConfigFactory,
    cryptography::{hashing::hashing_service::HashingService, rsa_jwt_factory::RsaJwtFactory},
    infrastructure::Infrastructure,
};

use crate::{
    auth::authentication_service::AuthenticationService,
    authorization::authorization_service::AuthorizationService,
    user::user_service::UserService,
    world::{world_member_service::WorldMemberService, world_service::WorldService},
};

pub struct Application {
    pub authentication_service: Arc<AuthenticationService>,
    pub authorization_service: Arc<AuthorizationService>,
    pub user_service: Arc<UserService>,
    pub world_service: Arc<WorldService>,
    pub world_member_service: Arc<WorldMemberService>,
}

impl Application {
    pub async fn new() -> Self {
        let infrastructure = Infrastructure::new().await;

        let config = ConfigFactory::build();

        let jwt_factory = Arc::new(RsaJwtFactory::new(
            config.auth_rsa_public_key.to_owned(),
            config.auth_rsa_private_key.to_owned(),
        ));

        let hashing_service = Arc::new(HashingService::new());

        let user_service = Arc::new(UserService::new(
            infrastructure.user_repository.clone(),
            hashing_service.clone(),
        ));

        let world_service = Arc::new(WorldService::new(
            infrastructure.world_repository.clone(),
            infrastructure.world_member_repository.clone(),
        ));

        let authentication_service = Arc::new(AuthenticationService::new(
            user_service.clone(),
            hashing_service.clone(),
        ));

        let authorization_service = Arc::new(AuthorizationService::new(
            jwt_factory.clone(),
            authentication_service.clone(),
        ));

        let world_member_service = Arc::new(WorldMemberService::new(
            infrastructure.world_member_repository.clone(),
        ));

        Self {
            authorization_service,
            authentication_service,
            user_service,
            world_service,
            world_member_service,
        }
    }
}
