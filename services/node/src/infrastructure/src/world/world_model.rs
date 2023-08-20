use domain::world::world::{World, WorldKind};
use sea_orm::prelude::*;

use crate::user::user_model;

#[derive(EnumIter, DeriveActiveEnum, Clone, Debug, PartialEq, Eq)]
#[sea_orm(rs_type = "String", db_type = "String(Some(1))")]
pub enum ModelWorldKind {
    #[sea_orm(string_value = "public")]
    Public,
    #[sea_orm(string_value = "private")]
    Private,
}

impl From<WorldKind> for ModelWorldKind {
    fn from(value: WorldKind) -> Self {
        match value {
            WorldKind::Public => ModelWorldKind::Public,
            WorldKind::Private => ModelWorldKind::Private,
        }
    }
}

impl Into<WorldKind> for ModelWorldKind {
    fn into(self) -> WorldKind {
        match self {
            ModelWorldKind::Public => WorldKind::Public,
            ModelWorldKind::Private => WorldKind::Private,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "world")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    pub name: String,
    pub description: String,
    pub kind: ModelWorldKind,
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
        World::from(
            self.id,
            self.name,
            self.description,
            self.kind.into(),
            self.owner_id,
        )
    }
}
