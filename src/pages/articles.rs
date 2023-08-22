use axum::{extract::Path, http, response::IntoResponse, Extension};
use sycamore::prelude::*;

use crate::{
    articles::{Article, ArticlesRepo},
    components::layout::{Header, Layout, MetaOGImage, SecondarySidebar, SidebarNavItem},
    root,
    sycamore::{Metatag, Title},
};

use super::auth::AuthContext;

pub async fn page_articles(
    uri: http::Uri,
    Extension(auth): Extension<AuthContext>,
    Extension(articles_repo): Extension<ArticlesRepo>,
) -> impl IntoResponse {
    root! {
        (uri, auth, articles_repo),
        Metatag(name="description".into(), attr:content="Antonio's articles on various topics related to software engineering and technology.") {}
        Title { "Articles - Antonio Pitasi" }
        Layout {
            Articles {}
        }
    }
}

#[derive(Props)]
pub struct ArticlesProps<'a, G: Html> {
    children: Option<Children<'a, G>>,
}

#[component]
pub fn Articles<'a, G: Html>(cx: Scope<'a>, props: ArticlesProps<'a, G>) -> View<G> {
    let repo = use_context::<ArticlesRepo>(cx);
    let v = View::new_fragment(repo
        .list()
        .into_iter()
        .map(|article| {
            let href = format!("/articles/{}", article.slug);
            let c = view! {
                cx,
                div(class="flex flex-col") {
                    span(class="font-semibold"){ (article.title.clone()) }
                    span(class="opacity-60"){ (article.datetime.format("%B %d, %Y").to_string()) }
                }
            };

            view! { cx, SidebarNavItem(href={href}){(c)} }
        })
        .collect()
    );

    let children = props.children.map(|children| {
        let inner = children.call(cx);
        view! { cx,
            div(class="absolute inset-0 lg:static empty:hidden") {
                (inner)
            }
        }
    });

    view! { cx,
        div(class="relative h-full w-full flex-row lg:grid lg:grid-cols-[20rem_minmax(0,1fr)]") {
            SecondarySidebar(title="Articles".into()) {(v)}
            (children)
        }
    }
}

pub async fn page_article(
    uri: http::Uri,
    Extension(auth): Extension<AuthContext>,
    Extension(articles_repo): Extension<ArticlesRepo>,
    Path(slug): Path<String>,
) -> impl IntoResponse {
    let article = articles_repo.get_article_by_slug(&slug).unwrap().clone();
    let title = format!("{} - Antonio Pitasi", article.title.clone());
    let og_image = format!(
        "https://anto.pt/articles/{}/social-image.png",
        article.slug.clone()
    );

    root! {
        (uri, auth, articles_repo),
        Metatag(name="description".into(), attr:content="Antonio's articles on various topics related to software engineering and technology.") {}
        Title { (title) }
        MetaOGImage(content=og_image) {}
        Layout {
            Articles {
                ArticleContent(a=article) {}
            }
        }
    }
}

#[derive(Props)]
pub struct ArticleContentProps {
    a: Article,
}

#[component]
fn ArticleContent<G: Html>(cx: Scope, props: ArticleContentProps) -> View<G> {
    let title = props.a.title.clone();

    view! { cx,
        main(class="typography relative min-h-full bg-floralwhite pb-24 lg:pb-0") {
            Header(title=title) {}
            article(class="w-full bg-floralwhite p-8") {
                div(class="mx-auto max-w-2xl") {
                    div(class="flex flex-col gap-3") {
                        h1(class="title font-neu font-semibold text-darkviolet text-4xl") {
                            (props.a.title)
                        }
                        div(class="flex flex-row") {
                            span(class="text-gray-500") {
                                "Written on "(props.a.datetime.format("%B %d, %Y").to_string())"."
                            }
                        }
                    }
                    div(class="mt-4", dangerously_set_inner_html=props.a.content.clone()) { }
                }
            }
        }
    }
}
