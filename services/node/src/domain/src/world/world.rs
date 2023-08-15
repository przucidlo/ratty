use crate::user::user::User;

pub struct World {
    id: u64,
    name: String,
    description: String,
    owner_id: u64,

    owner: Option<User>,
}

impl World {
    pub fn from(id: u64, name: String, description: String, owner_id: u64) -> Self {
        Self {
            id,
            name,
            description,
            owner_id,
            owner: None,
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn description(&self) -> &str {
        self.description.as_ref()
    }

    pub fn owner_id(&self) -> u64 {
        self.owner_id
    }

    pub fn set_owner(&mut self, owner: Option<User>) {
        self.owner = owner;
    }
}
