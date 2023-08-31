use domain::user::user::User;

use crate::errors::storage_error::StorageError;
use sqlx::MySqlPool;

pub struct UserRepository {
    pool: MySqlPool,
}

impl UserRepository {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }

    pub async fn find_one_by_username(&self, username: &str) -> Result<User, StorageError> {
        sqlx::query_as::<_, User>("SELECT * FROM user WHERE username = ? LIMIT 1")
            .bind(username.to_string())
            .fetch_one(&self.pool)
            .await
            .map_err(|_| StorageError::EntityNotFoundError)
    }

    pub async fn find_one_by_id(&self, id: u64) -> Result<User, StorageError> {
        sqlx::query_as::<_, User>("SELECT * FROM user WHERE id = ? LIMIT 1")
            .bind(id)
            .fetch_one(&self.pool)
            .await
            .map_err(|_| StorageError::EntityNotFoundError)
    }

    pub async fn insert(&self, user: User) -> Result<User, StorageError> {
        sqlx::query("INSERT INTO user (username, password) VALUES (?, ?)")
            .bind(user.username().to_string())
            .bind(user.password().to_string())
            .execute(&self.pool)
            .await
            .map_err(|_| StorageError::EntityNotFoundError)?;

        Self::find_one_by_username(&self, user.username()).await
    }
}
