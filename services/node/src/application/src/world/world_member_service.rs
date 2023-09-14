use std::sync::Arc;

use domain::{
    common::repository_error::RepositoryError,
    user::user::User,
    world::{world_member::WorldMember, world_member_repository::WorldMemberRepository},
};

pub struct WorldMemberService {
    world_member_repository: Arc<dyn WorldMemberRepository>,
}

impl WorldMemberService {
    pub fn new(world_member_repository: Arc<dyn WorldMemberRepository>) -> Self {
        Self {
            world_member_repository,
        }
    }

    pub async fn get_user_world_member(
        &self,
        user: User,
    ) -> Result<Vec<WorldMember>, RepositoryError> {
        self.world_member_repository
            .find_by_user_id(user.id())
            .await
    }
}
