use axum::{extract::Path, http, response::IntoResponse, Extension};
use leptos::*;

use crate::{
    articles::{Article, ArticlesRepo},
    components::layout::{Layout, MetaDescription, MetaOGImage, SecondarySidebar, SidebarNavItem},
    icons::Heart,
    leptos::Leptos,
    leptos_root,
};

use super::auth::AuthContext;

pub async fn page_articles(
    uri: http::Uri,
    Extension(auth): Extension<AuthContext>,
    Extension(articles_repo): Extension<ArticlesRepo>,
) -> impl IntoResponse {
    leptos_root! {
        (uri, auth, articles_repo),
        <MetaDescription content="Antonio's articles on various topics related to software engineering and technology." />
        <Layout title="Articles">
            <Articles />
        </Layout>
    }
}

#[component]
pub fn Articles(cx: Scope) -> impl IntoView {
    let repo = use_context::<ArticlesRepo>(cx).expect("ArticlesRepo not found");
    let v = repo
        .list()
        .into_iter()
        .map(|article| {
            let href = format!("/articles/{}", article.slug);
            let c = view! {
                cx,
                <div class="flex flex-col">
                    <span class="font-semibold">{ article.title.clone() }</span>
                    <span class="opacity-60">{ article.datetime.format("%B %d, %Y").to_string() }</span>
                </div>
            };

            view! { cx, <SidebarNavItem href={href}>{c}</SidebarNavItem> }
        })
        .collect_view(cx);

    let article = use_context::<Article>(cx);

    view! { cx,
        <div class="relative h-full w-full flex-row lg:grid lg:grid-cols-[20rem_minmax(0,1fr)]">
            <SecondarySidebar title="Articles">{v}</SecondarySidebar>
            {
                article.map(|article| view! { cx,
                    <div class="absolute inset-0 lg:static empty:hidden">
                        <ArticleContent a=article />
                    </div>
                })
            }
        </div>
    }
}

pub async fn page_article(
    uri: http::Uri,
    Extension(auth): Extension<AuthContext>,
    Extension(articles_repo): Extension<ArticlesRepo>,
    Path(slug): Path<String>,
) -> impl IntoResponse {
    let article = articles_repo.get_article_by_slug(&slug).unwrap().clone();
    let title = article.title.clone();
    let og_image = format!(
        "https://anto.pt/articles/{}/social-image.png",
        article.slug.clone()
    );

    leptos_root! {
        (uri, auth, articles_repo, article),
        <MetaDescription content="Antonio's articles on various topics related to software engineering and technology." />
        <MetaOGImage content=og_image />
        <Layout title=title>
            <Articles />
        </Layout>
    }
}

#[component]
fn ArticleContent(cx: Scope, a: Article) -> impl IntoView {
    view! { cx,
    //                 main class="typography relative min-h-full bg-floralwhite pb-24 lg:pb-0" {
    //                     (header(&uri, &a.title))
    //                     article class="w-full bg-floralwhite p-8" {
    //                       div class="mx-auto max-w-2xl" {
    //                         div class="flex flex-col gap-3" {
    //                             h1 class="font-neu text-bold text-darkviolet" { (a.title) }
    //                             div class="flex flex-row" {
    //                                 span class="text-gray-500" { "Written on " (a.datetime.format("%B %d, %Y")) "." }
    //                             }
    //                         }
    //
    //                         (a.content)
    //                       }
    //                     }
    //                 }
            <main class="typography relative min-h-full bg-floralwhite pb-24 lg:pb-0">
                <Header title=a.title.clone() />
                <article class="w-full bg-floralwhite p-8">
                    <div class="mx-auto max-w-2xl">
                        <div class="flex flex-col gap-3">
                            <h1 class="font-neu text-bold text-darkviolet">{a.title}</h1>
                            <div class="flex flex-row">
                                <span class="text-gray-500">Written on {a.datetime.format("%B %d, %Y").to_string()}.</span>
                            </div>
                        </div>
                        <div class="mt-4" inner_html=a.content />
                    </div>
                </article>
            </main>
        }
}

#[component]
fn Header(cx: Scope, title: String) -> impl IntoView {
    let uri = use_context::<http::Uri>(cx).expect("Uri not found");
    view! {cx,
        <header class="sticky top-0 z-10 flex w-full items-center justify-between gap-2 overflow-hidden border-b-2 border-black bg-yellow px-3 py-3 lg:justify-end lg:gap-4">
            <span id="header-title" class="line-clamp-1 text-ellipsis font-bold" style="opacity: 0; transform: translateY(30px) translateZ(0px);">
                {title}
            </span>
            <LazyHeartButton path=format!("/components/like-btn?url={}", uri.path()) />
        </header>
        <script>r#"
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
        "#</script>
    }
}

#[component]
fn LazyHeartButton(cx: Scope, path: String) -> impl IntoView {
    view! { cx,
        <button class="inline-flex items-center justify-center text-sm font-medium transition-colors focus:outline-none focus:ring-2 focus:ring-offset0 disabled:opacity-50 disabled:pointer-events-none bg-transparent hover:bg-slate-100 data-[state=open]:bg-transparent h-9 px-2 rounded-md"
            hx-get=path
            hx-trigger="load"
            hx-target="this"
            hx-swap="outerHTML">
            <div class="flex flex-row items-center justify-center gap-2 font-neu text-3xl font-bold">
                <Heart filled=false />
                <span>..</span>
            </div>
        </button>
    }
}
