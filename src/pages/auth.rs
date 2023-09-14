use std::env;

use async_trait::async_trait;
use axum::{
    extract::{FromRequestParts, Query},
    http::request::Parts,
    response::{IntoResponse, Redirect},
    Extension,
};
use axum_login::{
    axum_sessions::extractors::{ReadableSession, WritableSession},
    secrecy::SecretVec,
    AuthUser, PostgresStore,
};
use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AuthorizationCode, CsrfToken, Scope,
    TokenResponse,
};
use reqwest::StatusCode;
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Debug, Clone, PartialEq, PartialOrd, sqlx::Type)]
#[allow(dead_code)]
pub enum Role {
    User,
    Admin,
}

#[derive(Debug, Clone, sqlx::FromRow)]
#[allow(dead_code)]
pub struct User {
    pub id: i64,
    pub role: Role,
    pub email: String,
    pub username: Option<String>,
    pub picture: Option<String>,
}

impl AuthUser<i64, Role> for User {
    fn get_id(&self) -> i64 {
        self.id
    }

    fn get_password_hash(&self) -> SecretVec<u8> {
        SecretVec::new("password".into())
    }

    fn get_role(&self) -> Option<Role> {
        Some(self.role.clone())
    }
}

pub struct RequireAdmin(pub User);

#[async_trait]
impl<S> FromRequestParts<S> for RequireAdmin
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        #[cfg(debug_assertions)]
        {
            return Ok(RequireAdmin(User {
                id: 1,
                role: Role::Admin,
                email: "admin@localhost".into(),
                username: Some("admin".to_string()),
                picture: None,
            }));
        }

        let Extension(user): Extension<User> = Extension::from_request_parts(parts, state)
            .await
            .map_err(|_err| StatusCode::FORBIDDEN)?;

        if user
            .get_role()
            .map_or(false, |role| matches!(role, Role::Admin))
        {
            Ok(RequireAdmin(user))
        } else {
            Err(StatusCode::FORBIDDEN)
        }
    }
}

pub type AuthContext =
    axum_login::extractors::AuthContext<i64, User, PostgresStore<User, Role>, Role>;

#[derive(Deserialize)]
pub struct LoginQuery {
    redirect_to: Option<String>,
}

pub async fn login_handler(
    Extension(client): Extension<BasicClient>,
    mut session: WritableSession,
    Query(q): Query<LoginQuery>,
) -> impl IntoResponse {
    // Generate the authorization URL to which we'll redirect the user.
    let (auth_url, csrf_state) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("email".to_string()))
        .add_scope(Scope::new("profile".to_string()))
        .url();

    // Store the csrf_state in the session so we can assert equality in the callback
    session.insert("csrf_state", csrf_state).unwrap();
    if let Some(redirect_to) = q.redirect_to {
        session.insert("redirect_to", redirect_to).unwrap();
    }

    // Redirect to your oauth service
    Redirect::to(auth_url.as_ref())
}

pub async fn logout_handler(mut auth: AuthContext) -> impl IntoResponse {
    auth.logout().await;
    Redirect::to("/")
}

#[derive(Debug, Deserialize)]
pub struct AuthRequest {
    code: String,
    state: CsrfToken,
}

pub async fn oauth_callback_handler(
    mut auth: AuthContext,
    Query(query): Query<AuthRequest>,
    Extension(pool): Extension<PgPool>,
    Extension(oauth_client): Extension<BasicClient>,
    session: ReadableSession,
) -> impl IntoResponse {
    // Compare the csrf state in the callback with the state generated before the
    // request
    let original_csrf_state: CsrfToken = session.get("csrf_state").unwrap();
    let query_csrf_state = query.state.secret();
    let csrf_state_equal = original_csrf_state.secret() == query_csrf_state;
    let redirect_to: String = session.get("redirect_to").unwrap_or("/".into());

    drop(session);

    if !csrf_state_equal {
        println!("csrf state is invalid, cannot login",);

        // Return to some error
        return Redirect::to("/error");
    }

    println!("Getting oauth token");
    // Get an auth token
    let token = oauth_client
        .exchange_code(AuthorizationCode::new(query.code))
        .request_async(async_http_client)
        .await
        .unwrap();

    // Use auth token to fetch user info
    let user_client = reqwest::Client::new();
    let res = user_client
        .get(env::var("OAUTH_USERINFO_URL").expect("Missing OAUTH_USERINFO_URL!"))
        .headers(
            vec![(
                "Authorization".parse().unwrap(),
                format!("Bearer {}", token.access_token().secret())
                    .parse()
                    .unwrap(),
            )]
            .into_iter()
            .collect(),
        )
        .send()
        .await
        .unwrap();

    match res.status() {
        StatusCode::OK => {}
        _ => {
            return Redirect::to("/error");
        }
    };

    #[derive(Deserialize)]
    struct TokenResponse {
        email: String,
        username: Option<String>,
        picture: Option<String>,
    }

    let user_info = res.json::<TokenResponse>().await.unwrap();

    // Fetch the user and log them in
    let mut conn = pool.acquire().await.unwrap();
    let user = sqlx::query_as("select * from users where email = $1")
        .bind(&user_info.email)
        .fetch_one(&mut conn)
        .await;

    match user {
        Ok(user) => {
            auth.login(&user).await.unwrap();
        }
        Err(_) => {
            let user = sqlx::query_as(
                    "insert into users (email, username, picture, role) values ($1, $2, $3, $4) returning *",
                )
                .bind(&user_info.email)
                .bind(&user_info.username)
                .bind(&user_info.picture)
                .bind(Role::User)
                .fetch_one(&mut conn)
                .await
                .unwrap();
            auth.login(&user).await.unwrap();
        }
    }

    Redirect::to(&redirect_to)
}
