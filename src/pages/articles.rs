use axum::{extract::Path, http, response::IntoResponse, Extension};
use sycamore::prelude::*;

use crate::{
    articles::{Article, ArticlesRepo},
    components::layout::{Layout, MetaOGImage, SecondarySidebar, SidebarNavItem},
    icons::Heart,
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

#[derive(Props)]
pub struct HeaderProps {
    title: String,
}

#[component]
fn Header<G: Html>(cx: Scope, props: HeaderProps) -> View<G> {
    let uri = use_context::<http::Uri>(cx);
    view! {cx,
        header(class="sticky top-0 z-10 flex w-full items-center justify-between gap-2 overflow-hidden border-b-2 border-black bg-yellow px-3 py-3 lg:justify-end lg:gap-4") {
            span(id="header-title", class="line-clamp-1 text-ellipsis font-bold", style="opacity: 0; transform: translateY(30px) translateZ(0px);") {
                (props.title)
            }
            LazyHeartButton(path=format!("/components/like-btn?url={}", uri.path())) {}
        }
        script {(r#"
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
        "#)}
    }
}

#[derive(Props)]
pub struct LazyHeartButtonProps {
    path: String,
}

#[component]
fn LazyHeartButton<G: Html>(cx: Scope, props: LazyHeartButtonProps) -> View<G> {
    view! { cx,
        button(
            class="inline-flex items-center justify-center text-sm font-medium transition-colors focus:outline-none focus:ring-2 focus:ring-offset0 disabled:opacity-50 disabled:pointer-events-none bg-transparent hover:bg-slate-100 data-[state=open]:bg-transparent h-9 px-2 rounded-md",
            hx-get=props.path,
            hx-trigger="load",
            hx-target="this",
            hx-swap="outerHTML",
        ) {
            div(class="flex flex-row items-center justify-center gap-2 font-neu text-3xl font-bold") {
                Heart(filled=false) {}
                span { ".." }
            }
        }
    }
}
