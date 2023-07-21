use axum::{
    extract::Query,
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

pub type AuthContext =
    axum_login::extractors::AuthContext<i64, User, PostgresStore<User, Role>, Role>;

pub async fn login_handler(
    Extension(client): Extension<BasicClient>,
    mut session: WritableSession,
) -> impl IntoResponse {
    // Generate the authorization URL to which we'll redirect the user.
    let (auth_url, csrf_state) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("email".to_string()))
        .add_scope(Scope::new("profile".to_string()))
        .url();

    // Store the csrf_state in the session so we can assert equality in the callback
    session.insert("csrf_state", csrf_state).unwrap();

    // Redirect to your oauth service
    Redirect::to(auth_url.as_ref())
}

pub async fn logout_handler(mut auth: AuthContext) {
    dbg!("Logging out user: {}", &auth.current_user);
    auth.logout().await;
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
    println!("Running oauth callback {query:?}");
    // Compare the csrf state in the callback with the state generated before the
    // request
    let original_csrf_state: CsrfToken = session.get("csrf_state").unwrap();
    let query_csrf_state = query.state.secret();
    let csrf_state_equal = original_csrf_state.secret() == query_csrf_state;

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
        .get("https://poetic-camel-60.clerk.accounts.dev/oauth/userinfo")
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
    // {
    //   "object": "oauth_user_info",
    //   "instance_id": "ins_2ShOHnzrS6xAgpuGmo2Cf1CmZpN",
    //   "email": "antonio@pitasi.dev",
    //   "email_verified": true,
    //   "family_name": "Pitasi",
    //   "given_name": "Antonio",
    //   "name": "Antonio Pitasi",
    //   "username": "pitasi",
    //   "picture": "https://storage.googleapis.com/images.clerk.dev/oauth_github/img_2ShTHTM2KJoiONZ8SWVdb0p0NO0.jpeg",
    //   "user_id": "user_2ShTH93h3nTyatUwWj0pyuVW9GW"
    // }

    // Fetch the user and log them in
    let mut conn = pool.acquire().await.unwrap();
    let user = sqlx::query_as("select * from users where email = $1")
        .bind(&user_info.email)
        .fetch_one(&mut conn)
        .await;

    match user {
        Ok(user) => {
            println!("User found: {:?}", user);
            auth.login(&user).await.unwrap();
        }
        Err(e) => {
            println!("User not found: {:?}", e);
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
            println!("User created: {:?}", user);
            auth.login(&user).await.unwrap();
        }
    }

    Redirect::to("/")
}
