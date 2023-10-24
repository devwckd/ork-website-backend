use uuid::Uuid;

#[derive(Clone, Debug, serde::Serialize, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub role: i16,
}

pub type UserResult<R> = Result<R, UserError>;

#[derive(Debug, thiserror::Error)]
pub enum UserError {
    #[error("user already exists")]
    AlreadyExists,
    #[error("user not found")]
    NotFound,
    #[error("unknown error: {0}")]
    Unknown(String),
}

impl From<sqlx::Error> for UserError {
    fn from(value: sqlx::Error) -> Self {
        match value {
            sqlx::Error::Database(error) => {
                if error.message().contains("users_email_key") {
                    UserError::AlreadyExists
                } else {
                    UserError::Unknown(error.to_string())
                }
            }
            _ => UserError::Unknown(value.to_string()),
        }
    }
}
