use axum::{extract::Path, http, Extension};
use maud::{html, Markup};

use crate::{
    articles::ArticlesRepo,
    components::{is_active, root, secondary_sidebar, sidebar_nav_item, Meta},
    header,
};

use super::auth::AuthContext;

pub async fn page_articles(
    uri: http::Uri,
    Extension(auth): Extension<AuthContext>,
    articles_repo: Extension<ArticlesRepo>,
) -> Markup {
    root(
        &uri,
        Meta {
            title: Some("Articles"),
            description: Some("Antonio's articles on various topics related to software engineering and technology."),
            ..Default::default()
        },
        articles(&uri, articles_repo, None),
        auth.current_user,
    )
}

pub async fn page_article(
    auth: AuthContext,
    articles_repo: Extension<ArticlesRepo>,
    uri: http::Uri,
    Path(slug): Path<String>,
) -> Markup {
    let a = articles_repo.get_article_by_slug(&slug).unwrap().clone();

    root(
        &uri,
        Meta {
            title: Some(&a.title),
            social_image: Some(&format!(
                "https://anto.pt/articles/{}/social-image.png",
                a.slug
            )),
            ..Default::default()
        },
        articles(
            &uri,
            articles_repo,
            Some(html! {
                main class="typography relative min-h-full bg-floralwhite pb-24 lg:pb-0" {
                    (header(&uri, &a.title))
                    article class="w-full bg-floralwhite p-8" {
                      div class="mx-auto max-w-2xl" {
                        div class="flex flex-col gap-3" {
                            h1 class="font-neu text-bold text-darkviolet" { (a.title) }
                            div class="flex flex-row" {
                                span class="text-gray-500" { "Written on " (a.datetime.format("%B %d, %Y")) "." }
                            }
                        }

                        (a.content)
                      }
                    }
                }
            }),
        ),
        auth.current_user,
    )
}

fn articles(
    uri: &http::Uri,
    Extension(articles_repo): Extension<ArticlesRepo>,
    slot: Option<Markup>,
) -> Markup {
    html! {
        div class="relative h-full w-full flex-row lg:grid lg:grid-cols-[20rem_minmax(0,1fr)]" {
            (secondary_sidebar( html! {
                @for article in articles_repo.list() {
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
