use std::{error::Error, fmt::Display};

use sea_orm::DbErr;

#[derive(Debug)]
pub enum StorageError {
    EntityNotFoundError,
    QueryFailureError(DbErr),
}

impl Display for StorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StorageError::EntityNotFoundError => {
                write!(f, "User not found")
            }
            StorageError::QueryFailureError(_) => {
                write!(f, "Unexpected storage error")
            }
        }
    }
}

impl Error for StorageError {}

unsafe impl Send for StorageError {}
