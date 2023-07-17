use std::error::Error;

use domain::user::user::User;

pub struct UserService {}

impl UserService {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_user_by_username(&self, username: &str) -> Result<User, Box<dyn Error>> {
        Ok(User::new(username, "test"))
    }
}
