use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum RepositoryError {
    EntityNotFoundError,
    QueryFailureError(sqlx::Error),
}

impl Display for RepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RepositoryError::EntityNotFoundError => {
                write!(f, "Entity not found")
            }
            RepositoryError::QueryFailureError(_) => {
                write!(f, "Unexpected storage error")
            }
        }
    }
}

impl Error for RepositoryError {}

unsafe impl Send for RepositoryError {}
