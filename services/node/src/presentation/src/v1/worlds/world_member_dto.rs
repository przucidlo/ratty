use domain::world::world_member::WorldMember;
use garde::Validate;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Validate, Serialize)]
pub struct WorldMemberDto {
    #[garde(skip)]
    pub user_id: u64,

    #[garde(skip)]
    pub world_id: u64,
}

impl From<WorldMember> for WorldMemberDto {
    fn from(value: WorldMember) -> Self {
        Self {
            user_id: value.user_id(),
            world_id: value.world_id(),
        }
    }
}
