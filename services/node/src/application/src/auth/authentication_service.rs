use std::sync::Arc;

use domain::user::user::User;
use infrastructure::cryptography::hashing::hashing_service::HashingService;

use crate::user::user_service::UserService;

use super::errors::AuthenticationError;

pub struct AuthenticationService {
    hashing_service: Arc<HashingService>,
    user_service: Arc<UserService>,
}

impl AuthenticationService {
    pub fn new(user_service: Arc<UserService>, hashing_service: Arc<HashingService>) -> Self {
        AuthenticationService {
            user_service,
            hashing_service,
        }
    }

    pub async fn authenticate(
        &self,
        username: &str,
        password: &str,
    ) -> Result<User, AuthenticationError> {
        let user = self.user_service.get_user_by_username(username).await;

        match user {
            Ok(user) => match self.hashing_service.verify(password, user.password()) {
                Ok(is_password_correct) => {
                    if is_password_correct {
                        Ok(user)
                    } else {
                        Err(AuthenticationError::IncorrectCredentialsError)
                    }
                }
                Err(_) => Err(AuthenticationError::AuthenticationFailureError),
            },
            Err(_) => Err(AuthenticationError::AuthenticationFailureError),
        }
    }
}
