use crate::{errors::storage_error::StorageError, user::user_model};

use super::world_model;
use domain::world::world::World;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};

pub struct WorldRepository {
    db: DatabaseConnection,
}

impl WorldRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn find_by_id(&self, id: &u64) -> Result<World, StorageError> {
        let mut model: Vec<(world_model::Model, Vec<user_model::Model>)> =
            world_model::Entity::find_by_id(id.to_owned())
                .find_with_related(user_model::Entity)
                .all(&self.db)
                .await
                .map_err(|_| StorageError::EntityNotFoundError)?;

        match model.pop() {
            Some((model, mut users)) => {
                let mut world: World = model.into();

                if let Some(owner) = users.pop() {
                    world.set_owner(owner.into());
                }

                Ok(world)
            }
            None => Err(StorageError::EntityNotFoundError),
        }
    }

    pub async fn insert(&self, world: World) -> Result<World, StorageError> {
        let active_model = world_model::ActiveModel {
            name: Set(world.name().to_owned()),
            description: Set(world.description().to_owned()),
            ..Default::default()
        };

        let model = active_model
            .insert(&self.db)
            .await
            .map_err(|e| StorageError::QueryFailureError(Box::new(e)))?;

        Ok(model.into())
    }
}
