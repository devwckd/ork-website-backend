use crate::domains::error::ErrorResponse;
use crate::domains::session::{SessionError, SessionResult};
use crate::domains::user::User;
use crate::managers::session::SessionManager;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum_extra::extract::cookie::Cookie;
use axum_extra::extract::CookieJar;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use tracing::error;
use uuid::Uuid;

pub trait UserRole {
    fn check(level: i16) -> bool;
}

pub struct AnyUserRole;

impl UserRole for AnyUserRole {
    fn check(_level: i16) -> bool {
        true
    }
}

#[derive(Clone, Debug)]
pub struct AuthenticatedUser<UR: UserRole = AnyUserRole>(
    pub(super) User,
    pub(super) PhantomData<UR>,
);

impl<UR> Deref for AuthenticatedUser<UR>
where
    UR: UserRole,
{
    type Target = User;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<UR> DerefMut for AuthenticatedUser<UR>
where
    UR: UserRole,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<UR> From<AuthenticatedUser<UR>> for User
where
    UR: UserRole,
{
    fn from(value: AuthenticatedUser<UR>) -> Self {
        value.0
    }
}

#[async_trait::async_trait]
impl<S, UR> FromRequestParts<S> for AuthenticatedUser<UR>
where
    S: Send + Sync,
    UR: UserRole,
{
    type Rejection = AuthenticatedUserError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let jar = CookieJar::from_request_parts(parts, state).await.unwrap();

        let Some(session_cookie) = jar.get("ork_session_id") else {
            return Err(AuthenticatedUserError::Unauthenticated);
        };

        let session_id = Uuid::parse_str(session_cookie.value())
            .map_err(|err| AuthenticatedUserError::Unknown(err.to_string()))?;

        let session_manager: &SessionManager = parts.extensions.get().unwrap();
        let user = match session_manager.find_user_by_session_id(&session_id).await {
            Ok(user) => user,
            Err(SessionError::Invalid) => {
                let jar = jar.remove(Cookie::named("ork_session_id"));
                return Err(AuthenticatedUserError::Invalid(jar));
            }
            Err(SessionError::Unknown(err)) => return Err(AuthenticatedUserError::Unknown(err)),
        };

        if !UR::check(user.role) {
            return Err(AuthenticatedUserError::Forbidden);
        }

        Ok(AuthenticatedUser(user, PhantomData))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AuthenticatedUserError {
    #[error("session cookie not found")]
    Unauthenticated,
    #[error("no permission")]
    Forbidden,
    #[error("invalid session")]
    Invalid(CookieJar),
    #[error("unknown error: {0}")]
    Unknown(String),
}

impl IntoResponse for AuthenticatedUserError {
    fn into_response(self) -> Response {
        match self {
            AuthenticatedUserError::Unauthenticated => StatusCode::UNAUTHORIZED.into_response(),
            AuthenticatedUserError::Forbidden => StatusCode::FORBIDDEN.into_response(),
            AuthenticatedUserError::Invalid(jar) => (StatusCode::UNAUTHORIZED, jar).into_response(),
            AuthenticatedUserError::Unknown(err) => {
                error!("{:?}", err);
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    }
}
