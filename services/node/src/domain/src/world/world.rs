use std::str::FromStr;

use crate::user::user::User;
use sqlx::{mysql::MySqlRow, FromRow, Row};

use super::{world_error::JoinWorldError, world_kind::WorldKind, world_member::WorldMember};

#[derive(Debug)]
pub struct World {
    id: u64,
    name: String,
    description: String,
    kind: WorldKind,
    owner_id: u64,
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
        }
    }
}

impl World {
    pub fn new(name: String, description: String, kind: WorldKind, owner_id: u64) -> Self {
        Self {
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

    pub fn change_owner(&mut self, owner: User) {
        self.owner_id = owner.id();
    }

    pub fn join(&self, user_id: u64, is_world_member: bool) -> Result<WorldMember, JoinWorldError> {
        if is_world_member {
            return Err(JoinWorldError::WorldAlreadyJoinedError);
        }

        if matches!(self.kind, WorldKind::Private) && self.owner_id != user_id {
            return Err(JoinWorldError::WorldRequiresInvitationError);
        }

        Ok(WorldMember::new(user_id, self.id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_not_join_a_private_world() {
        let world = World {
            kind: WorldKind::Private,
            ..World::default()
        };

        assert_eq!(
            matches!(
                world.join(111, false).unwrap_err(),
                JoinWorldError::WorldRequiresInvitationError
            ),
            true
        );
    }

    #[test]
    fn should_join_private_world_only_as_owner() {
        let owner_id = 111;
        let world = World {
            kind: WorldKind::Private,
            owner_id,
            ..World::default()
        };

        assert_eq!(world.join(owner_id, false).is_ok(), true);
    }

    #[test]
    fn should_not_join_as_existing_member() {
        let world = World::default();

        assert_eq!(
            matches!(
                world.join(111, true).unwrap_err(),
                JoinWorldError::WorldAlreadyJoinedError
            ),
            true
        );
    }

    #[test]
    fn should_join_a_world() {
        let world = World {
            kind: WorldKind::Public,
            ..World::default()
        };

        assert_eq!(world.join(1, false).is_ok(), true);
    }
}
