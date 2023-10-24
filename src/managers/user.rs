use crate::domains::user::{User, UserResult};
use crate::repositories::user::UserRepository;

#[derive(Clone)]
pub struct UserManager {
    user_repository: UserRepository,
}

impl UserManager {
    pub fn new(user_repository: UserRepository) -> Self {
        Self { user_repository }
    }

    pub async fn find_by_email(&self, email: &String) -> UserResult<User> {
        self.user_repository.find_by_email(email).await
    }

    pub async fn insert(&self, user: &User) -> UserResult<()> {
        self.user_repository.insert(user).await
    }
}
