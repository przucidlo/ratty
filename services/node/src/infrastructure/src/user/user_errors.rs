use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum UserInfrastructureError {
    UserNotFoundError,
    UserStorageError,
}

impl Display for UserInfrastructureError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserInfrastructureError::UserNotFoundError => {
                write!(f, "User not found")
            }
            UserInfrastructureError::UserStorageError => {
                write!(f, "Unexpected storage error")
            }
        }
    }
}

impl Error for UserInfrastructureError {}
