use serde::Serialize;

use super::jwk::Jwk;

#[derive(Serialize)]
pub struct Jwks {
    keys: Vec<Jwk>,
}
