use std::sync::Arc;

use domain::user::user::User;
use infrastructure::{
    cryptography::hashing::hashing_service::HashingService,
    user::{user_errors::UserInfrastructureError, user_repository::UserRepository},
};

use super::user_errors::AddUserError;

pub struct UserService {
    user_repository: Arc<UserRepository>,
    hashing_service: Arc<HashingService>,
}

impl UserService {
    pub fn new(user_repository: Arc<UserRepository>, hashing_service: Arc<HashingService>) -> Self {
        Self {
            user_repository,
            hashing_service,
        }
    }

    pub async fn get_user_by_username(
        &self,
        username: &str,
    ) -> Result<User, UserInfrastructureError> {
        self.user_repository.find_by_username(username).await
    }

    pub async fn add_user(&self, username: &str, password: &str) -> Result<User, AddUserError> {
        let user = self.user_repository.find_by_username(username).await;
        let hashed_password = self.hashing_service.hash(&password).unwrap();

        match user {
            Ok(_) => Err(AddUserError::UserAlreadyExistsError),
            Err(_) => Ok(self
                .user_repository
                .insert(User::new(username, &hashed_password))
                .await
                .map_err(|_| AddUserError::UserRepositoryFailureError)?),
        }
    }
}
