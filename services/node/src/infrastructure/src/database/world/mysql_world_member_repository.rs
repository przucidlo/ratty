use async_trait::async_trait;
use domain::{
    common::repository_error::RepositoryError,
    world::{world_member::WorldMember, world_member_repository::WorldMemberRepository},
};
use sqlx::MySqlPool;

pub struct MySqlWorldMemberRepository {
    pool: MySqlPool,
}

impl MySqlWorldMemberRepository {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl WorldMemberRepository for MySqlWorldMemberRepository {
    async fn find_by_id(
        &self,
        user_id: u64,
        world_id: u64,
    ) -> Result<WorldMember, RepositoryError> {
        sqlx::query_as::<_, WorldMember>(
            "SELECT * FROM world_members WHERE user_id = ? AND world_id = ? LIMIT 1",
        )
        .bind(user_id)
        .bind(world_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|_| RepositoryError::EntityNotFoundError)
    }

    async fn find_by_user_id(&self, user_id: u64) -> Result<Vec<WorldMember>, RepositoryError> {
        sqlx::query_as::<_, WorldMember>("SELECT * FROM world_members WHERE user_id = ?")
            .bind(user_id)
            .fetch_all(&self.pool)
            .await
            .map_err(|_| RepositoryError::EntityNotFoundError)
    }

    async fn insert(&self, world_member: WorldMember) -> Result<WorldMember, RepositoryError> {
        let insert_result =
            sqlx::query("INSERT INTO world_members (user_id, world_id) VALUES (?, ?)")
                .bind(world_member.user_id())
                .bind(world_member.world_id())
                .execute(&self.pool)
                .await;

        if let Ok(_) = insert_result {
            return Self::find_by_id(&self, world_member.user_id(), world_member.world_id()).await;
        }

        Err(RepositoryError::QueryFailureError(
            insert_result.unwrap_err(),
        ))
    }
}
