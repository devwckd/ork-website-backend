use crate::domains::user::{User, UserError, UserResult};

#[derive(Clone)]
pub struct UserRepository {
    pg_pool: sqlx::PgPool,
}

impl UserRepository {
    pub fn new(pg_pool: sqlx::PgPool) -> Self {
        Self { pg_pool }
    }

    pub async fn find_by_email(&self, email: &String) -> UserResult<User> {
        let user: Option<User> = sqlx::query_as("SELECT * FROM users WHERE email = $1;")
            .bind(email)
            .fetch_optional(&self.pg_pool)
            .await?;

        user.ok_or(UserError::NotFound)
    }

    pub async fn insert(&self, user: &User) -> UserResult<()> {
        sqlx::query("INSERT INTO users(id, name, email, password_hash) VALUES ($1, $2, $3, $4);")
            .bind(&user.id)
            .bind(&user.name)
            .bind(&user.email)
            .bind(&user.password_hash)
            .execute(&self.pg_pool)
            .await?;

        Ok(())
    }
}
