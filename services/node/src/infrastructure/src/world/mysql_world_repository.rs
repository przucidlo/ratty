use async_trait::async_trait;
use domain::{
    common::repository_error::RepositoryError,
    world::{world::World, world_kind::WorldKind, world_repository::WorldRepository},
};
use sqlx::MySqlPool;

pub struct MySqlWorldRepository {
    pool: MySqlPool,
}

impl MySqlWorldRepository {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl WorldRepository for MySqlWorldRepository {
    async fn find_by_id(&self, id: u64) -> Result<World, RepositoryError> {
        sqlx::query_as::<_, World>("SELECT * FROM world WHERE id = ? LIMIT 1")
            .bind(id)
            .fetch_one(&self.pool)
            .await
            .map_err(|_| RepositoryError::EntityNotFoundError)
    }

    async fn find_all_by_kind(&self, kind: WorldKind) -> Result<Vec<World>, RepositoryError> {
        sqlx::query_as::<_, World>("SELECT * FROM world WHERE kind = ? ")
            .bind(kind.to_string())
            .fetch_all(&self.pool)
            .await
            .map_err(|e| RepositoryError::QueryFailureError(e))
    }

    async fn insert(&self, world: World) -> Result<World, RepositoryError> {
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

        Err(RepositoryError::QueryFailureError(
            insert_result.unwrap_err(),
        ))
    }
}
