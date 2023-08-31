use crate::errors::storage_error::StorageError;

use domain::world::world::{World, WorldKind};
use sqlx::MySqlPool;

pub struct WorldRepository {
    pool: MySqlPool,
}

impl WorldRepository {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }

    pub async fn find_by_id(&self, id: u64) -> Result<World, StorageError> {
        sqlx::query_as::<_, World>("SELECT * FROM world WHERE id = ? LIMIT 1")
            .bind(id)
            .fetch_one(&self.pool)
            .await
            .map_err(|_| StorageError::EntityNotFoundError)
    }

    pub async fn find_all_by_kind(&self, kind: WorldKind) -> Result<Vec<World>, StorageError> {
        sqlx::query_as::<_, World>("SELECT * FROM world WHERE kind = ? ")
            .bind(kind.to_string())
            .fetch_all(&self.pool)
            .await
            .map_err(|_| StorageError::EntityNotFoundError)
    }

    pub async fn insert(&self, world: World) -> Result<World, StorageError> {
        let insert_result = sqlx::query(
            "INSERT INTO world (name, description, kind, owner_id) VALUES (?, ?, ?, ?)",
        )
        .bind(world.name().to_string())
        .bind(world.description().to_string())
        .bind(world.kind().to_string())
        .bind(world.owner_id())
        .execute(&self.pool)
        .await;

        if let Ok(insert_result) = insert_result {
            return Self::find_by_id(&self, insert_result.last_insert_id()).await;
        }

        Err(StorageError::EntityNotFoundError)
    }
}
