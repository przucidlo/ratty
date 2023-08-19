use std::sync::Arc;

use domain::{user::user::User, world::world::World};
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

    pub fn create_world(
        &self,
        name: &str,
        description: &str,
        user: User,
    ) -> Result<World, StorageError> {
        let world = World::new(name.to_owned(), description.to_owned(), user);

        Ok(world)
    }
}
