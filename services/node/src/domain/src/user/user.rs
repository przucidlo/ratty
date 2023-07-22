#[derive(Clone)]
pub struct User {
    id: u64,
    username: String,
    password: String,
}

impl User {
    pub fn new(username: &str, password: &str) -> Self {
        Self {
            id: 0,
            username: username.to_owned(),
            password: password.to_owned(),
        }
    }

    pub fn new_existing(id: &u64, username: &str, password: &str) -> Self {
        Self {
            id: id.to_owned(),
            username: username.to_owned(),
            password: password.to_owned(),
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn password(&self) -> &str {
        &self.password
    }
}
