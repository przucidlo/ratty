use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum AuthenticationError {
    IncorrectCredentialsError,
    AuthenticationFailureError,
}

impl Display for AuthenticationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthenticationError::IncorrectCredentialsError => {
                write!(f, "Username or password is incorrect")
            }
            AuthenticationError::AuthenticationFailureError => {
                write!(f, "Authentication process has failed")
            }
        }
    }
}

impl Error for AuthenticationError {}
