use crate::domains::auth::{AuthResult, LoginData, RegisterData};
use crate::domains::session::Session;
use crate::domains::user::User;
use crate::extractors::authenticated_user::AuthenticatedUser;
use crate::managers::session::SessionManager;
use crate::managers::user::UserManager;
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use axum::extract::State;
use axum::response::Redirect;
use axum::routing::{get, post};
use axum::Json;
use axum_extra::extract::cookie::Cookie;
use axum_extra::extract::CookieJar;
use time::{Duration, OffsetDateTime};
use uuid::Uuid;
use validator::Validate;

pub fn router(user_manager: UserManager, session_manager: SessionManager) -> axum::Router {
    let auth_state = AuthState {
        user_manager,
        session_manager,
    };

    axum::Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/me", get(me))
        .route("/logout", get(logout))
        .with_state(auth_state)
}

async fn register(
    State(AuthState {
        user_manager,
        session_manager,
    }): State<AuthState>,
    jar: CookieJar,
    Json(data): Json<RegisterData>,
) -> AuthResult<(CookieJar, Redirect)> {
    data.validate()?;

    let user = User {
        id: Uuid::new_v4(),
        name: data.name,
        email: data.email,
        password_hash: hash_password(&data.password),
        role: 0,
    };

    user_manager.insert(&user).await?;

    let session = Session {
        id: Uuid::new_v4(),
        user,
        expires_at: OffsetDateTime::now_utc() + Duration::hours(12),
    };

    session_manager.insert(&session).await?;

    let jar = jar_with_session(jar, session);

    Ok((jar, Redirect::to("/auth/me")))
}

async fn login(
    State(AuthState {
        user_manager,
        session_manager,
    }): State<AuthState>,
    jar: CookieJar,
    Json(data): Json<LoginData>,
) -> AuthResult<(CookieJar, Redirect)> {
    data.validate()?;

    let user = user_manager.find_by_email(&data.email).await?;

    let hash = PasswordHash::new(&user.password_hash).unwrap();
    let argon2 = Argon2::default();
    argon2.verify_password(data.password.as_bytes(), &hash)?;

    let session = Session {
        id: Uuid::new_v4(),
        user,
        expires_at: OffsetDateTime::now_utc() + Duration::hours(12),
    };

    session_manager.insert(&session).await?;

    let jar = jar_with_session(jar, session);

    Ok((jar, Redirect::to("/auth/me")))
}

async fn me(user: AuthenticatedUser) -> AuthResult<Json<User>> {
    Ok(Json(user.into()))
}

async fn logout(_user: AuthenticatedUser, jar: CookieJar) -> AuthResult<CookieJar> {
    // TODO: remove from db
    let jar = jar.remove(
        Cookie::build("ork_session_id", "")
            .http_only(true)
            .secure(true)
            .path("/")
            .finish(),
    );
    Ok(jar)
}
#[derive(Clone)]
struct AuthState {
    user_manager: UserManager,
    session_manager: SessionManager,
}

fn hash_password(password: &String) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    argon2
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string()
}

fn jar_with_session(jar: CookieJar, session: Session) -> CookieJar {
    jar.add(
        Cookie::build("ork_session_id", session.id.to_string())
            .http_only(true)
            .secure(true)
            .expires(session.expires_at)
            .path("/")
            .finish(),
    )
}
