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

pub trait SessionState {
    fn session_manager(&self) -> &SessionManager;
}

pub trait Role {
    fn check(level: i16) -> bool;
}

pub struct AnyRole;

impl Role for AnyRole {
    fn check(_level: i16) -> bool {
        true
    }
}

#[derive(Clone, Debug)]
pub struct Authenticated<R: Role = AnyRole>(User, PhantomData<R>);

impl<R> Deref for Authenticated<R>
where
    R: Role,
{
    type Target = User;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<R> DerefMut for Authenticated<R>
where
    R: Role,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<R> Into<User> for Authenticated<R>
where
    R: Role,
{
    fn into(self) -> User {
        self.0
    }
}

#[async_trait::async_trait]
impl<S, R> FromRequestParts<S> for Authenticated<R>
where
    S: SessionState + Send + Sync,
    R: Role,
{
    type Rejection = AuthenticatedError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let jar = CookieJar::from_request_parts(parts, state).await.unwrap();

        let Some(session_cookie) = jar.get("ork_session_id") else {
            return Err(AuthenticatedError::Unauthenticated);
        };

        let session_id = Uuid::parse_str(session_cookie.value())
            .map_err(|err| AuthenticatedError::Unknown(err.to_string()))?;

        let session_manager = state.session_manager();
        let user = match session_manager.find_user_by_session_id(&session_id).await {
            Ok(user) => user,
            Err(SessionError::Invalid) => {
                let jar = jar.remove(Cookie::named("ork_session_id"));
                return Err(AuthenticatedError::Invalid(jar));
            }
            Err(SessionError::Unknown(err)) => return Err(AuthenticatedError::Unknown(err)),
        };

        if !R::check(user.role) {
            return Err(AuthenticatedError::Forbidden);
        }

        Ok(Authenticated(user, PhantomData))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AuthenticatedError {
    #[error("session cookie not found")]
    Unauthenticated,
    #[error("no permission")]
    Forbidden,
    #[error("invalid session")]
    Invalid(CookieJar),
    #[error("unknown error: {0}")]
    Unknown(String),
}

impl IntoResponse for AuthenticatedError {
    fn into_response(self) -> Response {
        match self {
            AuthenticatedError::Unauthenticated => StatusCode::UNAUTHORIZED.into_response(),
            AuthenticatedError::Forbidden => StatusCode::FORBIDDEN.into_response(),
            AuthenticatedError::Invalid(jar) => (StatusCode::UNAUTHORIZED, jar).into_response(),
            AuthenticatedError::Unknown(err) => {
                error!("{:?}", err);
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    }
}
