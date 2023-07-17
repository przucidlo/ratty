use std::sync::Arc;

use domain::user::user::User;

use crate::user::user_service::UserService;

use super::errors::AuthenticationError;

pub struct AuthenticationService {
    user_service: Arc<UserService>,
}

impl AuthenticationService {
    pub fn new(user_service: Arc<UserService>) -> Self {
        AuthenticationService { user_service }
    }

    pub fn authenticate(
        &self,
        username: &str,
        password: &str,
    ) -> Result<User, AuthenticationError> {
        let user = self.user_service.get_user_by_username(username);

        match user {
            Ok(user) => {
                if user.password() == password {
                    Ok(user)
                } else {
                    Err(AuthenticationError::IncorrectCredentialsError)
                }
            }
            Err(_) => Err(AuthenticationError::AuthenticationFailureError),
        }
    }
}
