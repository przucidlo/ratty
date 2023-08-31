use std::str::FromStr;

use crate::user::user::User;
use sqlx::{mysql::MySqlRow, FromRow, Row};

pub enum WorldKind {
    Public,
    Private,
}

pub struct World {
    id: u64,
    name: String,
    description: String,
    kind: WorldKind,
    owner_id: u64,

    owner: Option<User>,
}

impl FromRow<'_, MySqlRow> for World {
    fn from_row(row: &'_ MySqlRow) -> Result<Self, sqlx::Error> {
        let id: u64 = row.try_get("id")?;
        let name: String = row.try_get("name")?;
        let description: String = row.try_get("description")?;
        let kind: String = row.try_get("kind")?;
        let owner_id: u64 = row.try_get("owner_id")?;

        Ok(Self {
            id,
            name,
            description,
            kind: WorldKind::from_str(&kind).unwrap(),
            owner_id,
            owner: None,
        })
    }
}

impl Default for World {
    fn default() -> Self {
        Self {
            id: 0,
            name: "".to_owned(),
            description: "".to_owned(),
            kind: WorldKind::Private,
            owner_id: 0,
            owner: None,
        }
    }
}

impl FromStr for WorldKind {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "public" => Ok(Self::Public),
            "private" => Ok(Self::Private),
            _ => Err(()),
        }
    }
}

impl ToString for WorldKind {
    fn to_string(&self) -> String {
        match self {
            WorldKind::Public => "public".to_owned(),
            WorldKind::Private => "private".to_owned(),
        }
    }
}

impl World {
    pub fn new(name: String, description: String, kind: WorldKind, owner: User) -> Self {
        Self {
            name,
            description,
            kind,
            owner_id: owner.id(),
            owner: Some(owner),
            ..Self::default()
        }
    }

    pub fn from(
        id: u64,
        name: String,
        description: String,
        kind: WorldKind,
        owner_id: u64,
    ) -> Self {
        Self {
            id,
            name,
            description,
            kind,
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

    pub fn kind(&self) -> &WorldKind {
        &self.kind
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
