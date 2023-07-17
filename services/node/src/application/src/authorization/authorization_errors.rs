use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum AuthorizationError {
    AuthenticationFailureError,
    AuthorizationFailureError,
}

impl Display for AuthorizationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthorizationError::AuthenticationFailureError => {
                write!(f, "Username or password is incorrect.")
            }
            AuthorizationError::AuthorizationFailureError => {
                write!(f, "Authorization process failed.")
            }
        }
    }
}

impl Error for AuthorizationError {}
