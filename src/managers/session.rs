use crate::domains::session::{Session, SessionResult};
use crate::domains::user::User;
use crate::repositories::session::SessionRepository;
use uuid::Uuid;

#[derive(Clone)]
pub struct SessionManager {
    session_repository: SessionRepository,
}

impl SessionManager {
    pub fn new(session_repository: SessionRepository) -> Self {
        Self { session_repository }
    }

    pub async fn find_user_by_session_id(&self, session_id: &Uuid) -> SessionResult<User> {
        self.session_repository
            .find_user_by_session_id(session_id)
            .await
    }

    pub async fn insert(&self, session: &Session) -> SessionResult<()> {
        self.session_repository.insert(session).await
    }
}
