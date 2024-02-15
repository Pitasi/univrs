use axum::routing::post;
use axum::Form;
use axum::{extract::Query, http, response::IntoResponse, routing::get, Extension, Router};
use reqwest::Url;
use rscx::{context::provide_context, html};
use scraper::{Html, Selector};
use serde::Deserialize;

use crate::{bookmarks::BookmarksRepo, components::layout::Layout, meta::render_with_meta};

use crate::pages::auth::{AuthContext, RequireAdmin};

pub fn register(r: Router) -> Router {
    r.route("/add-bookmark", get(add_bookmark_handler))
        .route("/add-bookmark/form", get(form))
        .route("/add-bookmark/submit", post(submit))
}

pub async fn add_bookmark_handler(
    uri: http::Uri,
    RequireAdmin(_): RequireAdmin,
    Extension(auth): Extension<AuthContext>,
    Extension(bookmarks_repo): Extension<BookmarksRepo>,
) -> impl IntoResponse {
    render_with_meta(
        || {
            provide_context(uri);
            provide_context(auth);
            provide_context(bookmarks_repo);
        },
        || async {
            html! {
            <Layout title="Bookmarks - Antonio Pitasi">
               <div class="flex flex-col w-full">
                   <form class="flex flex-col" hx-get="./add-bookmark/form" hx-swap="outerHTML">
                       <input type="url" name="url" placeholder="URL" />
                       <button type="submit">Fetch metadata</button>
                   </form>
               </div>
            </Layout>
            }
        },
    )
    .await
}

#[derive(Deserialize)]
pub struct BookmarkPreview {
    url: String,
}

pub async fn form(
    RequireAdmin(_): RequireAdmin,
    Query(BookmarkPreview { url }): Query<BookmarkPreview>,
) -> impl IntoResponse {
    render_with_meta(
        || {},
        || async move {
            let html = reqwest::get(&url).await.unwrap().text().await.unwrap();
            let meta = extract_metadata(&url, &html);

            html! {
                <form class="flex flex-col" hx-post="./add-bookmark/submit">
                    <input type="text" name="slug" placeholder="slug" value={slugify(&meta.title)} />
                    <input type="url" name="url" placeholder="URL" value={url} />
                    <input type="text" name="title" placeholder="Title" value={meta.title} />
                    <input type="text" name="description" placeholder="Description" value={meta.description} />
                    <input type="url" name="favicon" placeholder="Favicon URL" value={meta.favicon} />
                    <input type="url" name="image" placeholder="Image URL" value={meta.image} />
                    <input type="date" name="posted_at" placeholder="Posted at" value={meta.published_at} />
                    <button type="submit">Save</button>
                </form>
            }
        },
    )
    .await
}

fn slugify(s: &str) -> String {
    s.split_whitespace()
        .take(10)
        .map(|s| s.to_lowercase())
        .map(|s| s.replace(|c: char| !c.is_ascii_alphanumeric(), ""))
        .filter(|s| !s.is_empty())
        .collect::<Vec<String>>()
        .join("-")
}

struct UrlMetadata {
    title: String,
    description: String,
    favicon: String,
    image: String,
    published_at: String,
}

fn extract_metadata(url: &str, html: &str) -> UrlMetadata {
    let dom = Html::parse_document(html);

    let selector = Selector::parse("title").unwrap();
    let title = dom
        .select(&selector)
        .next()
        .map(|title| title.text().collect::<Vec<&str>>().join(""))
        .unwrap_or(String::new());

    let selector = Selector::parse("link[rel=apple-touch-icon]").unwrap();
    let favicon = dom
        .select(&selector)
        .next()
        .map(|link| link.value().attr("href").unwrap_or(""))
        .map(|href| {
            if !href.starts_with("http") {
                let mut url = Url::parse(url).unwrap();
                url.set_path(href);
                url.to_string()
            } else {
                href.to_string()
            }
        });
    let favicon = match favicon {
        Some(favicon) => favicon,
        None => {
            let selector = Selector::parse("link[rel=icon]").unwrap();
            dom.select(&selector)
                .next()
                .map(|link| link.value().attr("href").unwrap_or(""))
                .map(|href| {
                    if !href.starts_with("http") {
                        let mut url = Url::parse(url).unwrap();
                        url.set_path(href);
                        url.to_string()
                    } else {
                        href.to_string()
                    }
                })
                .unwrap_or(String::new())
        }
    };

    let selector = Selector::parse("meta[name=description]").unwrap();
    let description = dom
        .select(&selector)
        .next()
        .map(|meta| meta.value().attr("content").unwrap_or(""))
        .unwrap_or("");

    let selector = Selector::parse("meta[name=\"og:image\"]").unwrap();
    let image = dom
        .select(&selector)
        .next()
        .map(|meta| meta.value().attr("content").unwrap_or(""))
        .unwrap_or("");

    UrlMetadata {
        title,
        description: description.to_string(),
        favicon: favicon.to_string(),
        image: image.to_string(),
        published_at: String::new(),
    }
}

#[derive(Deserialize)]
pub struct BookmarkSubmit {
    slug: String,
    url: String,
    title: String,
    description: String,
    favicon: String,
    image: String,
    posted_at: String,
}

pub async fn submit(
    RequireAdmin(_): RequireAdmin,
    Extension(bookmarks_repo): Extension<BookmarksRepo>,
    Form(BookmarkSubmit {
        slug,
        url,
        title,
        description,
        favicon,
        image,
        posted_at,
    }): Form<BookmarkSubmit>,
) -> impl IntoResponse {
    render_with_meta(
        || {},
        || async move {
            let d = chrono::NaiveDateTime::parse_from_str(
                &format!("{posted_at} 00:00:00"),
                "%Y-%m-%d %H:%M:%S",
            )
            .unwrap();
            let d = chrono::DateTime::from_utc(d, chrono::Utc);

            bookmarks_repo
                .add(
                    &slug,
                    &url,
                    &title,
                    &description,
                    (!favicon.is_empty()).then(|| favicon.as_str()),
                    (!image.is_empty()).then(|| image.as_str()),
                    &d,
                )
                .await
                .unwrap();
            html! {
                <p>big success</p>
                <p>
                    <a href=format!("/bookmarks/{slug}")>go check result</a>
                </p>
            }
        },
    )
    .await
}
