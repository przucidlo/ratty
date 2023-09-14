use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum JoinWorldError {
    WorldNotFoundError,
    WorldAlreadyJoinedError,
    WorldJoinFailedError,
    WorldRequiresInvitationError,
}

impl Error for JoinWorldError {}

impl Display for JoinWorldError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JoinWorldError::WorldNotFoundError => {
                write!(f, "World with given id does not exists.")
            }
            JoinWorldError::WorldAlreadyJoinedError => {
                write!(f, "World already joined.")
            }
            JoinWorldError::WorldJoinFailedError => {
                write!(f, "Could not join the world, due to server error.")
            }
            JoinWorldError::WorldRequiresInvitationError => {
                write!(f, "Could not join the world, due to server error.")
            }
        }
    }
}
