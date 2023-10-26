use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

use crate::domains::error::ErrorResponse;
use crate::domains::organization::{Organization, OrganizationError};
use crate::domains::organization_member::{OrganizationMember, OrganizationMemberError};
use axum::extract::{FromRequestParts, Path};
use axum::http::request::Parts;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use tracing::error;
use uuid::Uuid;

use crate::domains::user::User;
use crate::extractors::authenticated_user::{
    AnyUserRole, AuthenticatedUser, AuthenticatedUserError, UserRole,
};
use crate::managers::organization::OrganizationManager;
use crate::managers::organization_member::OrganizationMemberManager;

pub trait OrganizationRole {
    fn check(level: i16) -> bool;
}

pub struct AnyOrganizationRole;

impl OrganizationRole for AnyOrganizationRole {
    fn check(_level: i16) -> bool {
        true
    }
}

#[derive(Clone, Debug)]
pub struct AuthenticatedOrgMember<
    UR: UserRole = AnyUserRole,
    OR: OrganizationRole = AnyOrganizationRole,
>(
    User,
    Organization,
    OrganizationMember,
    PhantomData<(UR, OR)>,
);

impl<UR, OR> AuthenticatedOrgMember<UR, OR>
where
    UR: UserRole,
    OR: OrganizationRole,
{
    pub fn org(&self) -> &Organization {
        &self.1
    }

    pub fn role(&self) -> i16 {
        self.2.role
    }
}

impl<UR, OR> Deref for AuthenticatedOrgMember<UR, OR>
where
    UR: UserRole,
    OR: OrganizationRole,
{
    type Target = User;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<UR, OR> DerefMut for AuthenticatedOrgMember<UR, OR>
where
    UR: UserRole,
    OR: OrganizationRole,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<UR, OR> Into<User> for AuthenticatedOrgMember<UR, OR>
where
    UR: UserRole,
    OR: OrganizationRole,
{
    fn into(self) -> User {
        self.0
    }
}

#[async_trait::async_trait]
impl<S, UR, OR> FromRequestParts<S> for AuthenticatedOrgMember<UR, OR>
where
    S: Send + Sync,
    UR: UserRole + Send + Sync,
    OR: OrganizationRole,
{
    type Rejection = AuthenticatedOrgMemberError;

    async fn from_request_parts(mut parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let AuthenticatedUser(user, _) =
            AuthenticatedUser::<UR>::from_request_parts(parts, state).await?;

        let Path((organization_id,)): Path<(Uuid,)> =
            Path::from_request_parts(parts, state).await.unwrap();

        let organization_manager: &OrganizationManager = parts.extensions.get().unwrap();
        let organization = organization_manager.find_by_id(&organization_id).await?;

        let organization_manager_member: &OrganizationMemberManager =
            parts.extensions.get().unwrap();
        let organization_member = organization_manager_member
            .find_by_user_id(&organization_id, &user.id)
            .await?;

        if !OR::check(organization_member.role) {
            return Err(AuthenticatedOrgMemberError::Forbidden);
        }

        Ok(AuthenticatedOrgMember(
            user,
            organization,
            organization_member,
            PhantomData,
        ))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AuthenticatedOrgMemberError {
    #[error("no permission")]
    Forbidden,
    #[error("user error: {0}")]
    UserAuthenticatedError(#[from] AuthenticatedUserError),
    #[error("organization error: {0}")]
    OrganizationError(#[from] OrganizationError),
    #[error("organization member error: {0}")]
    OrganizationMemberError(#[from] OrganizationMemberError),
}

impl IntoResponse for AuthenticatedOrgMemberError {
    fn into_response(self) -> Response {
        match self {
            AuthenticatedOrgMemberError::Forbidden => StatusCode::FORBIDDEN.into_response(),
            AuthenticatedOrgMemberError::UserAuthenticatedError(err) => err.into_response(),
            AuthenticatedOrgMemberError::OrganizationError(err) => match err {
                OrganizationError::NotFound => err.into_response(),
                OrganizationError::Unknown(_) => {
                    ErrorResponse::of(StatusCode::INTERNAL_SERVER_ERROR, "internal server error")
                        .into_response()
                }
                _ => unreachable!(),
            },
            AuthenticatedOrgMemberError::OrganizationMemberError(err) => match err {
                OrganizationMemberError::NotFound => {
                    ErrorResponse::of(StatusCode::NOT_FOUND, "organization not found")
                        .into_response()
                }
            },
        }
    }
}
