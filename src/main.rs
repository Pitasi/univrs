#![warn(clippy::pedantic)]

pub mod articles;
pub mod components;
pub mod hash;
pub mod icons;
pub mod images;
pub mod markdown;
pub mod pages;
pub mod rsc;
pub mod social_img;
pub mod sycamore;

use axum::{
    http::Request,
    middleware::{self, Next},
    response::Response,
    routing::{get, post},
    Extension, Router,
};
use axum_login::{
    axum_sessions::{async_session::MemoryStore as SessionMemoryStore, SameSite, SessionLayer},
    AuthLayer, PostgresStore,
};
use oauth2::{
    basic::BasicClient, AuthType, AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl,
};
use pages::auth::{AuthContext, User};
use rand::Rng;
use sqlx::postgres::PgPoolOptions;
use std::{
    env,
    error::Error,
    net::{IpAddr, SocketAddr},
    str::FromStr,
};
use tower_http::{
    compression::CompressionLayer,
    services::ServeDir,
    trace::{self, TraceLayer},
};
#[cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use tower_livereload::LiveReloadLayer;
use tracing::Level;

use crate::{articles::ArticlesRepo, pages::auth::Role};

fn not_htmx<Body>(req: &Request<Body>) -> bool {
    !req.headers().contains_key("hx-request")
}

async fn set_cache_headers<B>(req: Request<B>, next: Next<B>) -> Response {
    let mut res = next.run(req).await;
    res.headers_mut()
        .insert("cache-control", "public,max-age=604800".parse().unwrap());
    res
}

fn build_oauth_client() -> BasicClient {
    let client_id = env::var("CLIENT_ID").expect("Missing CLIENT_ID!");
    let client_secret = env::var("CLIENT_SECRET").expect("Missing CLIENT_SECRET!");
    let redirect_url = env::var("CALLBACK_URL").expect("Missing CALLBACK_URL!");
    let auth_url_str = env::var("OAUTH_AUTH_URL").expect("Missing OAUTH_AUTH_URL!");
    let token_url_str = env::var("OAUTH_TOKEN_URL").expect("Missing OAUTH_TOKEN_URL!");

    let auth_url =
        AuthUrl::new(auth_url_str.to_string()).expect("Invalid authorization endpoint URL");
    let token_url = TokenUrl::new(token_url_str.to_string()).expect("Invalid token endpoint URL");

    BasicClient::new(
        ClientId::new(client_id),
        Some(ClientSecret::new(client_secret)),
        auth_url,
        Some(token_url),
    )
    .set_redirect_uri(RedirectUrl::new(redirect_url).unwrap())
    .set_auth_type(AuthType::RequestBody)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // tracing
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    // auth
    let secret = env::var("AUTH_SECRET").map_or_else(
        |_| rand::thread_rng().gen::<[u8; 64]>().to_vec(),
        String::into_bytes,
    );

    let session_store = SessionMemoryStore::new();
    let session_layer = SessionLayer::new(session_store, &secret)
        .with_secure(false)
        .with_same_site_policy(SameSite::Lax);

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(
            &env::var("DB_URL")
                .unwrap_or("postgres://postgres:mysecretpassword@localhost/univrs".into()),
        )
        .await?;

    sqlx::migrate!().run(&pool).await?;

    let user_store = PostgresStore::<User, Role>::new(pool.clone());
    let auth_layer = AuthLayer::new(user_store, &secret);

    let files = ServeDir::new("static")
        .precompressed_br()
        .precompressed_gzip();

    let oauth_client = build_oauth_client();

    let articles_repo = ArticlesRepo::new();

    let app = Router::new()
        .nest_service("/static", files)
        .layer(middleware::from_fn(set_cache_headers))
        .route("/auth/login", get(pages::auth::login_handler))
        .route("/auth/callback", get(pages::auth::oauth_callback_handler))
        .route("/auth/logout", get(pages::auth::logout_handler))
        .route("/", get(pages::homepage::handler))
        .route("/articles", get(pages::articles::page_articles))
        .route("/articles/:slug", get(pages::articles::page_article))
        .route(
            "/articles/:slug/social-image.png",
            get(social_img::social_image_article),
        );

    let components = Router::new()
        .route("/like-btn", get(components::heart::handler_get))
        .route("/like-btn", post(components::heart::handler_post));

    let router = Router::new().nest("/", app).nest("/components", components);

    let router = router
        .layer(Extension(pool))
        .layer(Extension(oauth_client))
        .layer(Extension(articles_repo))
        .layer(auth_layer)
        .layer(session_layer)
        .layer(CompressionLayer::new())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        );

    #[cfg(debug_assertions)]
    let router = router.layer(tower_livereload::LiveReloadLayer::new().request_predicate(not_htmx));
    #[cfg(debug_assertions)]
    println!("Live reload enabled");

    let addr: SocketAddr = (
        IpAddr::from_str(&env::var("HOST").unwrap_or("127.0.0.1".into()))?,
        3000,
    )
        .into();

    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await?;

    Ok(())
}
