use crate::validation::one_of::one_of;
use domain::world::world::World;
use garde::Validate;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Validate, Serialize)]
pub struct WorldDto {
    #[garde(skip)]
    pub id: Option<u64>,

    #[garde(length(min = 3))]
    pub name: String,

    #[garde(length(min = 1))]
    pub description: Option<String>,

    #[garde(custom(|value, _| {
        one_of(vec!["public", "private"], value)
    }))]
    pub kind: String,
}

impl From<World> for WorldDto {
    fn from(value: World) -> Self {
        Self {
            id: Some(value.id().to_owned()),
            name: value.name().to_owned(),
            description: Some(value.description().to_owned()),
            kind: value.kind().to_owned().to_string(),
        }
    }
}
