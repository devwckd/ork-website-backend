use crate::domains::user::{User, UserError};
use sqlx::Error;
use time::OffsetDateTime;
use tracing::error;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Session {
    pub id: Uuid,
    pub user: User,
    pub expires_at: OffsetDateTime,
}

pub type SessionResult<R> = Result<R, SessionError>;

#[derive(Debug, thiserror::Error)]
pub enum SessionError {
    #[error("invalid session")]
    Invalid,
    #[error("unknown error: {0}")]
    Unknown(String),
}

impl From<sqlx::Error> for SessionError {
    fn from(value: Error) -> Self {
        error!("{:?}", value);
        todo!()
    }
}
