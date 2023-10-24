use crate::domains::session::{Session, SessionError, SessionResult};
use crate::domains::user::{User, UserError};
use time::{Duration, OffsetDateTime};
use uuid::Uuid;

#[derive(Clone)]
pub struct SessionRepository {
    pg_pool: sqlx::PgPool,
}

impl SessionRepository {
    pub fn new(pg_pool: sqlx::PgPool) -> Self {
        Self { pg_pool }
    }

    pub async fn find_user_by_session_id(&self, session_id: &Uuid) -> SessionResult<User> {
        let user: Option<User> = sqlx::query_as(
            r#"
        SELECT 
            sessions.user_id AS s_user_id,
            users.* 
        FROM sessions
            INNER JOIN users ON users.id = sessions.user_id
        WHERE sessions.id = $1
            LIMIT 1;
        "#,
        )
        .bind(session_id)
        .fetch_optional(&self.pg_pool)
        .await?;

        user.ok_or(SessionError::Invalid)
    }
    pub async fn insert(&self, session: &Session) -> SessionResult<()> {
        sqlx::query("INSERT INTO sessions(id, user_id, expires_at) VALUES ($1, $2, $3);")
            .bind(&session.id)
            .bind(&session.user.id)
            .bind(&session.expires_at)
            .execute(&self.pg_pool)
            .await?;

        Ok(())
    }
}
