use crate::{errors::storage_error::StorageError, user::user_model};

use super::world_model::{self, ModelWorldKind};
use domain::world::world::{World, WorldKind};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};

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

    pub async fn find_all_public(&self) -> Result<Vec<World>, StorageError> {
        let mut model = world_model::Entity::find()
            .filter(world_model::Column::Kind.eq(WorldKind::Public.to_string()))
            .all(&self.db)
            .await
            .map_err(|err| StorageError::QueryFailureError(err))?;

        Ok(model.drain(..).map(|m| m.into()).collect())
    }

    pub async fn insert(&self, world: World) -> Result<World, StorageError> {
        let active_model = world_model::ActiveModel {
            name: Set(world.name().to_owned()),
            description: Set(world.description().to_owned()),
            owner_id: Set(world.owner_id().to_owned()),
            kind: Set(ModelWorldKind::from(world.kind().to_owned())),
            ..Default::default()
        };

        let model = active_model
            .insert(&self.db)
            .await
            .map_err(|e| StorageError::QueryFailureError(e))?;

        Ok(model.into())
    }
}
