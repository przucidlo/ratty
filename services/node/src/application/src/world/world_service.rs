use std::sync::Arc;

use domain::{
    common::repository_error::RepositoryError,
    user::user::User,
    world::{
        world::World, world_error::JoinWorldError, world_kind::WorldKind,
        world_member::WorldMember, world_member_repository::WorldMemberRepository,
        world_repository::WorldRepository,
    },
};

pub struct WorldService {
    world_repository: Arc<dyn WorldRepository>,
    world_member_repository: Arc<dyn WorldMemberRepository>,
}

unsafe impl Send for WorldService {}

impl WorldService {
    pub fn new(
        world_repository: Arc<dyn WorldRepository>,
        world_member_repository: Arc<dyn WorldMemberRepository>,
    ) -> Self {
        Self {
            world_repository,
            world_member_repository,
        }
    }

    pub async fn get_public_worlds(&self) -> Result<Vec<World>, RepositoryError> {
        self.world_repository
            .find_all_by_kind(WorldKind::Public)
            .await
    }

    pub async fn create_world(
        &self,
        name: &str,
        description: &str,
        kind: WorldKind,
        user: User,
    ) -> Result<World, RepositoryError> {
        let mut world = World::new(name.to_owned(), description.to_owned(), kind, user.id());

        world = self.world_repository.insert(world).await?;

        let world_member = world.join(user.id(), false).unwrap();

        self.world_member_repository.insert(world_member).await?;

        Ok(world)
    }

    pub async fn join_world(
        &self,
        user: User,
        world_id: u64,
    ) -> Result<WorldMember, JoinWorldError> {
        let world = self.world_repository.find_by_id(world_id).await;

        match world {
            Ok(world) => {
                let existing_world_membership = self
                    .world_member_repository
                    .find_by_id(user.id(), world_id)
                    .await;

                let mut world_member = world.join(user.id(), existing_world_membership.is_ok())?;

                world_member = self
                    .world_member_repository
                    .insert(world_member)
                    .await
                    .map_err(|_| JoinWorldError::WorldJoinFailedError)?;

                Ok(world_member)
            }
            Err(_) => Err(JoinWorldError::WorldNotFoundError),
        }
    }
}
