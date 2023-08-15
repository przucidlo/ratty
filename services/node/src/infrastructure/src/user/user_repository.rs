use domain::user::user::User;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};

use crate::errors::storage_error::StorageError;

use super::user_model;

pub struct UserRepository {
    connection: DatabaseConnection,
}

impl UserRepository {
    pub fn new(connection: DatabaseConnection) -> Self {
        Self { connection }
    }

    pub async fn find_one_by_username(&self, username: &str) -> Result<User, StorageError> {
        let model = user_model::Entity::find()
            .filter(user_model::Column::Username.contains(username))
            .one(&self.connection)
            .await
            .map_err(|_| StorageError::EntityNotFoundError)?;

        match model {
            Some(model) => Ok(model.into()),
            None => Err(StorageError::EntityNotFoundError),
        }
    }

    pub async fn find_one_by_id(&self, id: u64) -> Result<User, StorageError> {
        let model = user_model::Entity::find_by_id(id)
            .one(&self.connection)
            .await
            .map_err(|_| StorageError::EntityNotFoundError)?;

        match model {
            Some(model) => Ok(model.into()),
            None => Err(StorageError::EntityNotFoundError),
        }
    }

    pub async fn insert(&self, user: User) -> Result<User, StorageError> {
        let active_model = user_model::ActiveModel {
            username: Set(user.username().to_owned()),
            password: Set(user.password().to_owned()),
            ..Default::default()
        };

        let model = active_model
            .insert(&self.connection)
            .await
            .map_err(|_| StorageError::EntityNotFoundError)?;

        Ok(model.into())
    }
}
