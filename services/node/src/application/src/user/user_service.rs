use std::{error::Error, sync::Arc};

use domain::user::user::User;
use infrastructure::{
    cryptography::hashing::hashing_service::HashingService, user::user_repository::UserRepository,
};

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

    pub async fn get_user_by_username(&self, username: &str) -> Result<User, Box<dyn Error>> {
        let user = self.user_repository.find_by_username(username).await;

        match user {
            Ok(user) => Ok(user),
            Err(_) => todo!(),
        }
    }

    pub async fn add_user(&self, username: &str, password: &str) -> Result<User, Box<dyn Error>> {
        let user = self.user_repository.find_by_username(username).await;
        let hashed_password = self.hashing_service.hash(&password).unwrap();

        match user {
            Ok(_) => todo!(),
            Err(_) => Ok(self
                .user_repository
                .insert(User::new(username, &hashed_password))
                .await?),
        }
    }
}
