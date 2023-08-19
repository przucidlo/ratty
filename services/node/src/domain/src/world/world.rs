use crate::user::user::User;

pub struct World {
    id: u64,
    name: String,
    description: String,
    owner_id: u64,

    owner: Option<User>,
}

impl Default for World {
    fn default() -> Self {
        Self {
            id: 0,
            name: "".to_owned(),
            description: "".to_owned(),
            owner_id: 0,
            owner: None,
        }
    }
}

impl World {
    pub fn new(name: String, description: String, owner: User) -> Self {
        Self {
            name,
            description,
            owner_id: owner.id(),
            owner: Some(owner),
            ..Self::default()
        }
    }

    pub fn from(id: u64, name: String, description: String, owner_id: u64) -> Self {
        Self {
            id,
            name,
            description,
            owner_id,
            ..Self::default()
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
        if let Some(owner) = &owner {
            self.owner_id = owner.id();
        }

        self.owner = owner;
    }
}
