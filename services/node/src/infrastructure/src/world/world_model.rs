use domain::world::world::World;
use sea_orm::prelude::*;

use crate::user::user_model;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "world")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    pub name: String,
    pub description: String,
    pub owner_id: u64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "user_model::Entity",
        from = "Column::OwnerId",
        to = "user_model::Column::Id"
    )]
    User,
}

impl Related<user_model::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl Into<World> for Model {
    fn into(self) -> World {
        World::from(self.id, self.name, self.description, self.owner_id)
    }
}
