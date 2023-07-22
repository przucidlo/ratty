use std::sync::Arc;

use infrastructure::cryptography::rsa_jwt_factory::RsaJwtFactory;

use crate::auth::authentication_service::AuthenticationService;

use super::{authorization_errors::AuthorizationError, user_token_payload::UserTokenPayload};

pub struct AuthorizationService {
    jwt_factory: Arc<RsaJwtFactory>,
    authentication_service: Arc<AuthenticationService>,
}

impl AuthorizationService {
    pub fn new(
        jwt_factory: Arc<RsaJwtFactory>,
        authentication_service: Arc<AuthenticationService>,
    ) -> Self {
        Self {
            jwt_factory,
            authentication_service,
        }
    }

    pub async fn authorize(
        &self,
        username: &str,
        password: &str,
    ) -> Result<String, AuthorizationError> {
        match &self
            .authentication_service
            .authenticate(username, password)
            .await
        {
            Ok(user) => {
                let payload = UserTokenPayload {
                    user_id: user.id().to_owned(),
                };

                Ok(self
                    .jwt_factory
                    .encode(payload)
                    .map_err(|_| AuthorizationError::AuthenticationFailureError))?
            }
            Err(_) => Err(AuthorizationError::AuthenticationFailureError),
        }
    }
}
