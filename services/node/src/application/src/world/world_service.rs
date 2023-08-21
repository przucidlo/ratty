use std::sync::Arc;

use domain::{
    user::user::User,
    world::world::{World, WorldKind},
};
use infrastructure::{
    errors::storage_error::StorageError, world::world_repository::WorldRepository,
};

pub struct WorldService {
    world_repository: Arc<WorldRepository>,
}

impl WorldService {
    pub fn new(world_repository: Arc<WorldRepository>) -> Self {
        Self { world_repository }
    }

    pub async fn get_public_worlds(&self) -> Result<Vec<World>, StorageError> {
        self.world_repository.find_all_public().await
    }

    pub async fn create_world(
        &self,
        name: &str,
        description: &str,
        kind: WorldKind,
        user: User,
    ) -> Result<World, StorageError> {
        let mut world = World::new(name.to_owned(), description.to_owned(), kind, user);

        world = self.world_repository.insert(world).await?;

        Ok(world)
    }
}
