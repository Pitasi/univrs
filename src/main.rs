pub mod articles;
pub mod components;
pub mod hash;
pub mod icons;
pub mod images;
pub mod markdown;
pub mod pages;
pub mod rsc;
pub mod sstyle;
pub mod xmarkdown;

use axum::{
    extract::Query,
    http::{self, HeaderMap, Request},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::{get, post},
    Extension, Form, Router,
};
use axum_login::{
    axum_sessions::{async_session::MemoryStore as SessionMemoryStore, SameSite, SessionLayer},
    AuthLayer, PostgresStore,
};
use maud::{html, Markup, PreEscaped};
use oauth2::{
    basic::BasicClient, AuthType, AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl,
};
use pages::auth::{AuthContext, User};
use rand::Rng;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, PgPool};
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // tracing
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    // auth
    let secret = rand::thread_rng().gen::<[u8; 64]>();

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
        let token_url =
            TokenUrl::new(token_url_str.to_string()).expect("Invalid token endpoint URL");

        BasicClient::new(
            ClientId::new(client_id),
            Some(ClientSecret::new(client_secret)),
            auth_url,
            Some(token_url),
        )
        .set_redirect_uri(RedirectUrl::new(redirect_url).unwrap())
        .set_auth_type(AuthType::RequestBody)
    }

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
        .route("/articles/:slug", get(pages::articles::page_article));

    let components = Router::new()
        .route("/like-btn", get(page_get_like_btn))
        .route("/like-btn", post(page_post_like_btn));

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

#[derive(Serialize, Deserialize)]
pub struct LikeBtnPayload {
    pub url: String,
}

fn lazy_component(component_path: &str) -> Markup {
    // todo: generalize this into suspense()
    html! {
        button
            class="inline-flex items-center justify-center text-sm font-medium transition-colors focus:outline-none focus:ring-2 focus:ring-offset0 disabled:opacity-50 disabled:pointer-events-none bg-transparent hover:bg-slate-100 data-[state=open]:bg-transparent h-9 px-2 rounded-md"
            hx-get=(component_path)
            hx-payload="{}"
            hx-trigger="load"
            hx-target="this"
            hx-swap="outerHTML" {
                div class="flex flex-row items-center justify-center gap-2 font-neu text-3xl font-bold" {
                    (icons::heart(false))
                    span { "..." }
                }
        }
    }
}

async fn like_btn(pool: PgPool, user: Option<User>, url: &str, act: bool) -> Markup {
    let mut conn = pool.acquire().await.unwrap();
    let mut has_like = match &user {
        Some(u) => sqlx::query_as::<_, Like>(
            r#"
            select * from likes
            where user_id = $1
            and url = $2
        "#,
        )
        .bind(u.id)
        .bind(url)
        .fetch_one(&mut conn)
        .await
        .is_ok(),
        None => false,
    };

    if act && user.is_some() {
        let u = user.unwrap();
        if has_like {
            sqlx::query(
                r#"
                    delete from likes
                    where user_id = $1
                    and url = $2
                "#,
            )
            .bind(u.id)
            .bind(url)
            .execute(&mut conn)
            .await
            .unwrap();
            has_like = false;
        } else {
            sqlx::query(
                r#"
                    insert into likes (user_id, url)
                    values ($1, $2)
                "#,
            )
            .bind(u.id)
            .bind(url)
            .execute(&mut conn)
            .await
            .unwrap();
            has_like = true;
        }
    }

    let count: i64 = sqlx::query_scalar(
        r#"
            select count(*) from likes
            where url = $1
        "#,
    )
    .bind(url)
    .fetch_one(&pool)
    .await
    .unwrap();

    let payload = serde_json::to_string(&LikeBtnPayload {
        url: url.to_string(),
    })
    .unwrap();

    html! {
        button
            class="inline-flex items-center justify-center text-sm font-medium transition-colors focus:outline-none focus:ring-2 focus:ring-offset0 disabled:opacity-50 disabled:pointer-events-none bg-transparent hover:bg-slate-100 data-[state=open]:bg-transparent h-9 px-2 rounded-md"
            hx-post="/components/like-btn"
            hx-trigger="click"
            hx-target="this"
            hx-swap="outerHTML"
            hx-vals=(payload)
            data-loading-disable {
                div class="flex flex-row items-center justify-center gap-2 font-neu text-3xl font-bold" {
                    (icons::heart(has_like))
                    span { (count.to_string()) }
                }
        }
    }
}

fn header(uri: &http::Uri, title: &str) -> Markup {
    html! {
    header class="sticky top-0 z-10 flex w-full items-center justify-between gap-2
        overflow-hidden border-b-2 border-black bg-yellow px-3 py-3 lg:justify-end lg:gap-4" {
        span
            id="header-title"
            class="line-clamp-1 text-ellipsis font-bold"
            style="opacity: 0; transform: translateY(30px) translateZ(0px);" {
            (title)
        }
        (lazy_component(&("/components/like-btn?url=".to_string() + uri.path())))
    }
    script {(PreEscaped(r#"
var animation = anime({
  targets: '#header-title',
  translateY: 0,
  opacity: 1,
  easing: 'easeInOutSine',
  autoplay: false
});

window.addEventListener("scroll", () => {
    const scrollPercent = Math.min(window.scrollY, 200) / 200;
    animation.seek(scrollPercent * animation.duration);
}, false);
    "#))}
    }
}

#[derive(Debug, sqlx::FromRow)]
pub struct Like {
    pub id: i64,
    pub user_id: i64,
    pub url: String,
}

#[derive(Deserialize)]
struct PageLikeBtnQuery {
    url: String,
}

async fn page_get_like_btn(
    auth: AuthContext,
    Extension(pool): Extension<PgPool>,
    query: Query<PageLikeBtnQuery>,
) -> impl IntoResponse {
    like_btn(pool, auth.current_user, &query.url, false).await
}

async fn page_post_like_btn(
    auth: AuthContext,
    Extension(pool): Extension<PgPool>,
    Form(payload): Form<LikeBtnPayload>,
) -> impl IntoResponse {
    let mut header_map = HeaderMap::new();
    if auth.current_user.is_none() {
        header_map.insert("HX-Redirect", "/auth/login".parse().unwrap());
    }

    (
        header_map,
        like_btn(pool, auth.current_user, &payload.url, true).await,
    )
}
