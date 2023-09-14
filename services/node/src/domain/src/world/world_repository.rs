use async_trait::async_trait;

use crate::common::repository_error::RepositoryError;

use super::{world::World, world_kind::WorldKind};

#[async_trait]
pub trait WorldRepository: Send + Sync {
    async fn insert(&self, world: World) -> Result<World, RepositoryError>;
    async fn find_by_id(&self, id: u64) -> Result<World, RepositoryError>;
    async fn find_all_by_kind(&self, kind: WorldKind) -> Result<Vec<World>, RepositoryError>;
}
