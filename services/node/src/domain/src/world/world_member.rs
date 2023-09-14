use sqlx::FromRow;

#[derive(FromRow, Debug)]
pub struct WorldMember {
    user_id: u64,
    world_id: u64,
}

impl WorldMember {
    pub fn new(user_id: u64, world_id: u64) -> Self {
        Self { user_id, world_id }
    }

    pub fn user_id(&self) -> u64 {
        self.user_id
    }

    pub fn world_id(&self) -> u64 {
        self.world_id
    }
}
