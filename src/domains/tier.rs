use uuid::Uuid;

#[derive(Clone, Debug, serde::Serialize, sqlx::FromRow)]
pub struct Tier {
    pub id: Uuid,
    pub slug: String,
}
