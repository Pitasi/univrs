pub mod articles;
pub mod icons;
pub mod markdown;
pub mod rsc;

use articles::{get_article_by_slug, list_articles};
use axum::{
    async_trait,
    extract::{FromRequestParts, Path},
    http::{self, request::Parts, Request, StatusCode},
    routing::{get, post},
    Extension, Router,
};
use axum_login::{
    axum_sessions::{async_session::MemoryStore as SessionMemoryStore, SessionLayer},
    secrecy::SecretVec,
    AuthLayer, AuthUser, PostgresStore,
};
use maud::{html, Markup, PreEscaped};
use rand::Rng;
use sqlx::postgres::PgPoolOptions;
use std::{
    env,
    error::Error,
    net::{IpAddr, SocketAddr},
    str::FromStr,
};
use tower_http::services::ServeDir;
#[cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use tower_livereload::LiveReloadLayer;

#[derive(Debug, Clone, PartialEq, PartialOrd, sqlx::Type)]
#[allow(dead_code)]
enum Role {
    User,
    Admin,
}

#[derive(Debug, Clone, sqlx::FromRow)]
struct User {
    id: i64,
    password_hash: Vec<u8>,
    role: Role,
    name: String,
}

impl AuthUser<i64, Role> for User {
    fn get_id(&self) -> i64 {
        self.id
    }

    fn get_password_hash(&self) -> SecretVec<u8> {
        SecretVec::new(self.password_hash.clone().into())
    }

    fn get_role(&self) -> Option<Role> {
        Some(self.role.clone())
    }
}

/// Example how to create an Admin user guard
/// Can be modified to support any type of permission
struct RequireAdmin(User);
struct RequireUser(User);
struct MaybeUser(Option<User>);

#[async_trait]
impl<S> FromRequestParts<S> for RequireAdmin
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
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

#[async_trait]
impl<S> FromRequestParts<S> for RequireUser
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Extension(user): Extension<User> = Extension::from_request_parts(parts, state)
            .await
            .map_err(|_err| StatusCode::FORBIDDEN)?;

        if user
            .get_role()
            .map_or(false, |role| matches!(role, Role::User))
        {
            Ok(RequireUser(user))
        } else {
            Err(StatusCode::FORBIDDEN)
        }
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for MaybeUser
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let x: Result<Extension<User>, _> = Extension::from_request_parts(parts, state).await;
        match x {
            Ok(Extension(user)) => Ok(MaybeUser(Some(user))),
            Err(err) => {
                println!("error: {:?}", err);
                Ok(MaybeUser(None))
            }
        }
    }
}

type AuthContext = axum_login::extractors::AuthContext<i64, User, PostgresStore<User, Role>, Role>;

fn not_htmx<Body>(req: &Request<Body>) -> bool {
    !req.headers().contains_key("hx-request")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // auth
    let secret = rand::thread_rng().gen::<[u8; 64]>();

    let session_store = SessionMemoryStore::new();
    let session_layer = SessionLayer::new(session_store, &secret).with_secure(false);

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(
            &env::var("DB_URL")
                .unwrap_or("postgres://postgres:mysecretpassword@localhost/univrs".into()),
        )
        .await?;

    sqlx::migrate!().run(&pool).await?;

    let user_store = PostgresStore::<User, Role>::new(pool);
    let auth_layer = AuthLayer::new(user_store, &secret);

    async fn login_handler(mut auth: AuthContext) {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(
                &env::var("DB_URL")
                    .unwrap_or("postgres://postgres:mysecretpassword@localhost/univrs".into()),
            )
            .await
            .unwrap();
        let mut conn = pool.acquire().await.unwrap();
        let user: User = sqlx::query_as("select * from users where id = 1")
            .fetch_one(&mut conn)
            .await
            .unwrap();
        auth.login(&user).await.unwrap();
    }

    async fn logout_handler(mut auth: AuthContext) {
        dbg!("Logging out user: {}", &auth.current_user);
        auth.logout().await;
    }

    let files = ServeDir::new("static");
    let app = Router::new()
        .route("/login", get(login_handler))
        .route("/logout", get(logout_handler))
        .route("/", get(page_home))
        .route("/articles", get(page_articles))
        .route("/articles/:slug", get(page_article))
        .nest_service("/static", files);

    let components = Router::new().route("/like-btn", post(page_like_btn));

    let router = Router::new()
        .nest("/", app)
        .nest("/components", components)
        .layer(auth_layer)
        .layer(session_layer);

    #[cfg(debug_assertions)]
    let router = router.layer(tower_livereload::LiveReloadLayer::new().request_predicate(not_htmx));

    let addr: SocketAddr = (
        IpAddr::from_str(&env::var("HOST").unwrap_or("127.0.0.1".into()))?,
        3000,
    )
        .into();

    println!("Listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await?;

    Ok(())
}

fn sidebar_header(img_src: Option<&str>, title: &str) -> Markup {
    html! {
        header class="flex h-10 w-full flex-row items-center gap-2" {
            @if let Some(src) = img_src {
                div ."flex" ."h-8" ."w-8" ."shrink-0" ."items-center" ."justify-center" ."rounded-xl" {
                    img class="drop-shadow-border max-w-full w-auto h-auto" src=(src) alt=(title) loading="lazy" decoding="async";
                }
            } @else {
                div ."h-10" ."w-4";
            }
            div ."flex" ."flex-col" ."font-neu" ."text-md" ."font-bold" ."leading-none" ."tracking-tight" ."text-black" {
                span { (title) }
            }
        }
    }
}

fn sidebar_nav(slot: Markup) -> Markup {
    html! {
        ul ."space-y-4" {
            (slot)
        }
    }
}

fn sidebar_nav_item(href: &str, icon: &Option<Markup>, slot: Markup, active: bool) -> Markup {
    html! {
        li {
            a href=(href) class="
                rounded-md text-sm font-medium transition-colors focus:outline-none
                focus:ring-offset0 disabled:opacity-50 disabled:pointer-events-none
                data-[state=open]:bg-slate-100 py-2 px-4 flex h-auto w-full flex-row items-center
                justify-start ring-inset ring-lightviolet ring-offset-transparent focus:ring-1
                data-active:translate-x-0 data-active:translate-y-0 data-active:border-black
                data-active:bg-yellow data-active:shadow-none -translate-x-0.5 -translate-y-0.5
                border-2 border-black bg-white shadow-neu-2 hover:translate-x-0 hover:translate-y-0
                hover:shadow-none
            " data-active=(active) {
                @if let Some(icon) = icon {
                    span class="mr-2 h-4 w-4" { (icon) }
                }
                (slot)
            }
        }
    }
}

fn root_sidebar(uri: &http::Uri) -> Markup {
    let nav = vec![
        ("Home", "/", Some(icons::home())),
        ("Articles", "/articles", Some(icons::pen())),
    ];
    html! {
        div class="
            hidden w-48 shrink-0 bg-lightviolet bg-pattern-hideout
            lg:flex lg:flex-col lg:justify-between sticky bottom-0 top-0
            max-h-screen space-y-8 overflow-auto border-black p-4
            lg:border-r-2
        " {
            div class="space-y-8" {
                (sidebar_header(Some("/static/bulb.png"), "Antonio Pitasi"))
                (sidebar_nav(html! {
                    @for (name, href, icon) in nav.iter() {
                        (sidebar_nav_item(href, icon, html! {
                            (name)
                        }, is_active(uri.path(), href)))
                    }
                })
                )
            }
        }
    }
}

fn is_active(path: &str, href: &str) -> bool {
    match href {
        "/" => path == "/",
        _ => path.starts_with(href),
    }
}

struct Meta {
    title: Option<String>,
}

impl Default for Meta {
    fn default() -> Meta {
        Meta { title: None }
    }
}

fn root(uri: &http::Uri, meta: Meta, slot: Markup) -> Markup {
    let title = match meta.title {
        Some(title) => format!("{} - Antonio Pitasi", title),
        None => "Antonio Pitasi".into(),
    };
    html! {
        (maud::DOCTYPE)
        html lang="en" ."bg-floralwhite" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                title { (title) }
                link rel="stylesheet" type="text/css" href="/static/style.css";
                link rel="stylesheet" type="text/css" href="/static/tailwind.css";
            }
            body class="flex min-h-screen" hx-ext="loading-states" {
              script src="/static/anime.min.js" {}
              script src="https://unpkg.com/htmx.org@1.9.2" integrity="sha384-L6OqL9pRWyyFU3+/bjdSri+iIphTN/bvYyM37tICVyOJkWZLpP2vGn6VUEXgzg6h" crossorigin="anonymous" {}
              script src="https://unpkg.com/htmx.org/dist/ext/loading-states.js" {}
              .flex ."flex-1" .flex-row {
                (root_sidebar(uri))
                (slot)
              }
            }
        }
    }
}

async fn page_home(uri: http::Uri) -> Markup {
    root(
        &uri,
        Meta::default(),
        html! {
                main class="typography mx-auto my-20 max-w-2xl space-y-16 px-6 text-liver lg:px-14" {
                    section {
            p {"
I'm Antonio, a backend software engineer. I'm passionate about distributed
systems and clean maintainable software. In my free time, I organize events
with the local community I founded: pisa.dev.
"}

            p {"
I'm currently working on exciting technology at Qredo. We aim to decentralize
the private keys for your cryptocurrencies using our dMPC solution.
"}

            p {"
Before that, I worked at Ignite (also known as Tendermint), the company that
first created Proof-of-Stake and Cosmos SDK. My role was Senior Backend
Engineer for the (now defunct) Emeris.
"}

            p {"
Before diving into cryptocurrencies tech, I've cutted my teeth in fast-paced
startups where I helped shaping products such as Traent and Zerynth.
"}

            p {"
Sometimes I have over-engineering tendencies, such as my personal website.
Most of the times I'm harmless though.
"}
            }
            a href="/articles" { "Read my articles" }
            }
        },
    )
}

fn secondary_sidebar(slot: Markup) -> Markup {
    html! {
        div class="
sticky bottom-0 top-0 max-h-screen w-full space-y-8 overflow-auto border-black p-4 lg:border-r-2 min-h-full shrink-0 bg-pattern-hideout pb-24 lg:block lg:min-h-0 lg:pb-0
        " {
            div class="space-y-8" {
                (sidebar_header(None, "Articles"))
                (sidebar_nav(slot))
            }
        }
    }
}

fn articles(uri: &http::Uri, slot: Option<Markup>) -> Markup {
    html! {
        div class="relative h-full w-full flex-row lg:grid lg:grid-cols-[20rem_minmax(0,1fr)]" {
            (secondary_sidebar( html! {
                @for article in list_articles() {
                    @let href = format!("/articles/{}", article.slug);
                    (sidebar_nav_item(&href, &None, html! {
                        div class="flex flex-col" {
                            span class="font-semibold" { (article.title) }
                            span class="opacity-60" { (article.datetime.format("%B %d, %Y")) }
                        }
                    }, is_active(uri.path(), &href)))
                }
            }))
            @if let Some(slot) = slot {
                div class="absolute inset-0 lg:static" {
                    (slot)
                }
            }
        }
    }
}

async fn page_articles(uri: http::Uri) -> Markup {
    root(
        &uri,
        Meta {
            title: Some("Articles".into()),
        },
        articles(&uri, None),
    )
}

fn like_btn(user: Option<User>, slug: &str) -> Markup {
    let count = match user {
        Some(_) => 42,
        None => 0,
    };
    let payload = format!(r#"{{"slug": "{slug}"}}"#);
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
                    (icons::heart())
                    span { (count.to_string()) }
                }
        }
    }
}

fn header(user: Option<User>) -> Markup {
    html! {
    header class="sticky top-0 z-10 flex w-full items-center justify-between gap-2
        overflow-hidden border-b-2 border-black bg-yellow px-3 py-3 lg:justify-end lg:gap-4" {
        span
            id="header-title"
            class="line-clamp-1 text-ellipsis font-bold"
            style="opacity: 0; transform: translateY(30px) translateZ(0px);" {
            "Astro: writing static websites like it’s 2023"
        }
        (like_btn(user, "sluggg"))
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

async fn page_like_btn(auth: AuthContext) -> Markup {
    like_btn(auth.current_user, "sluggg")
}

async fn page_article(auth: AuthContext, uri: http::Uri, Path(slug): Path<String>) -> Markup {
    let a = get_article_by_slug(slug).unwrap();
    let title = &a.title;

    root(
        &uri,
        Meta {
            title: Some(title.into()),
        },
        articles(
            &uri,
            Some(html! {
                main class="typography relative min-h-full bg-floralwhite pb-24 lg:pb-0" {
                    (header(auth.current_user))
                    article class="w-full bg-floralwhite p-8" {
                      div class="mx-auto max-w-2xl" {
                        h1 class="title-neu" { (title) }
                        (PreEscaped(a.content))
                      }
                    }
                }
            }),
        ),
    )
}
