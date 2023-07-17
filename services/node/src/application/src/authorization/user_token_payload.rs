use serde::Serialize;

#[derive(Serialize)]
pub struct UserTokenPayload {
    pub user_id: usize,
}
