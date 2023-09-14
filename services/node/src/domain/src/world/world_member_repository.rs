use async_trait::async_trait;

use crate::common::repository_error::RepositoryError;

use super::world_member::WorldMember;

#[async_trait]
pub trait WorldMemberRepository: Send + Sync {
    async fn insert(&self, world_member: WorldMember) -> Result<WorldMember, RepositoryError>;
    async fn find_by_id(&self, user_id: u64, world_id: u64)
        -> Result<WorldMember, RepositoryError>;
    async fn find_by_user_id(&self, user_id: u64) -> Result<Vec<WorldMember>, RepositoryError>;
}
