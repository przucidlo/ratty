use std::error::Error;

use bcrypt::DEFAULT_COST;

pub struct HashingService {}

impl HashingService {
    pub fn new() -> Self {
        Self {}
    }

    pub fn hash(&self, password: &str) -> Result<String, Box<dyn Error>> {
        bcrypt::hash(password, DEFAULT_COST).map_err(|e| e.into())
    }

    pub fn verify(&self, password: &str, hashed_password: &str) -> Result<bool, Box<dyn Error>> {
        bcrypt::verify(password, hashed_password).map_err(|e| e.into())
    }
}
