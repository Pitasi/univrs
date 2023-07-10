pub mod articles;
pub mod icons;
pub mod markdown;
pub mod rsc;

use articles::{get_article_by_slug, list_articles};
use axum::{extract::Path, http, routing::get, Router};
use maud::{html, Markup, PreEscaped};

use std::{
    env,
    error::Error,
    net::{IpAddr, SocketAddr},
    str::FromStr,
};
use tower_http::services::ServeDir;
use tower_livereload::LiveReloadLayer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let dev = env::var("DEV").unwrap_or("".into()) == "true";

    let files = ServeDir::new("static");
    let mut app = Router::new()
        .route("/", get(page_home))
        .route("/articles", get(page_articles))
        .route("/articles/:slug", get(page_article))
        .nest_service("/static", files);
    if dev {
        println!("Enabling live reload");
        app = app.layer(LiveReloadLayer::new());
    }

    let addr: SocketAddr = (
        IpAddr::from_str(&env::var("HOST").unwrap_or("127.0.0.1".into()))?,
        3000,
    )
        .into();

    println!("Listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

fn sidebar_header(img_src: Option<&str>, title: &str) -> Markup {
    html! {
        header ."flex" ."h-10" ."w-full" ."flex-row" ."items-center" ."gap-1" {
            @if let Some(src) = img_src {
                div ."flex" ."h-10" ."w-10" ."shrink-0" ."items-center" ."justify-center" ."rounded-xl" {
                    img class="drop-shadow-border max-w-full w-auto h-auto" src=(src) alt=(title) loading="lazy" decoding="async";
                }
            } @else {
                div ."h-10" ."w-4";
            }
            div ."flex" ."flex-col" ."font-neu" ."text-xl" ."font-bold" ."leading-none" ."tracking-tight" ."text-black" {
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
            body .flex .min-h-screen  {
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

async fn page_article(uri: http::Uri, Path(slug): Path<String>) -> Markup {
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
