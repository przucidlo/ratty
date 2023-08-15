use domain::user::user::User;
use sea_orm::prelude::*;

use crate::world::world_model;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    pub username: String,
    pub password: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "world_model::Entity")]
    World,
}

impl Related<world_model::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::World.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl Into<User> for Model {
    fn into(self) -> User {
        User::from(self.id, self.username, self.password)
    }
}

impl Into<Option<User>> for Model {
    fn into(self) -> Option<User> {
        Some(User::from(self.id, self.username, self.password))
    }
}
