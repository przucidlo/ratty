use std::sync::Arc;

use infrastructure::{
    config::config_factory::ConfigFactory, cryptography::rsa_jwt_factory::RsaJwtFactory,
};

use crate::{
    auth::authentication_service::AuthenticationService,
    authorization::authorization_service::AuthorizationService, user::user_service::UserService,
};

pub struct Application {
    pub authentication_service: Arc<AuthenticationService>,
    pub authorization_service: Arc<AuthorizationService>,
}

impl Default for Application {
    fn default() -> Self {
        let config = ConfigFactory::build();

        let jwt_factory = Arc::new(RsaJwtFactory::new(
            config.auth_rsa_public_key.to_owned(),
            config.auth_rsa_private_key.to_owned(),
        ));
        let user_service = Arc::new(UserService::new());

        let authentication_service = Arc::new(AuthenticationService::new(user_service.clone()));
        let authorization_service = Arc::new(AuthorizationService::new(
            jwt_factory.clone(),
            authentication_service.clone(),
        ));

        Self {
            authorization_service,
            authentication_service,
        }
    }
}
