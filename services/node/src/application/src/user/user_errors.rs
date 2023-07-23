use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum AddUserError {
    UserAlreadyExistsError,
    UserRepositoryFailureError,
}

impl Error for AddUserError {}

impl Display for AddUserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AddUserError::UserAlreadyExistsError => {
                write!(f, "Username is already taken")
            }
            AddUserError::UserRepositoryFailureError => {
                write!(f, "User could not be saved")
            }
        }
    }
}
