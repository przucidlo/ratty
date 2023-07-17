#[warn(dead_code)]
pub struct User {
    id: usize,
    username: String,
    password: String,
}

impl User {
    pub fn new(username: &str, password: &str) -> Self {
        Self {
            id: 0,
            username: username.to_owned(),
            password: username.to_owned(),
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn password(&self) -> &str {
        &self.password
    }
}
