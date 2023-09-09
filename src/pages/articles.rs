use atom_syndication::{
    ContentBuilder, EntryBuilder, FeedBuilder, LinkBuilder, PersonBuilder, TextBuilder,
};
use axum::{
    extract::Path,
    http,
    response::{IntoResponse, Response},
    Extension,
};
use rscx::{
    component,
    context::{expect_context, provide_context},
    html, props, CollectFragmentAsync,
};

use crate::{
    articles::{Article, ArticlesRepo},
    components::layout::{
        Header, HeaderProps, Layout, LayoutProps, MetaOGImage, MetaOGImageProps, SecondarySidebar,
        SecondarySidebarProps, SidebarNavItem, SidebarNavItemProps,
    },
    meta::{render_with_meta, Dedup, DedupProps},
};

use super::auth::AuthContext;

pub async fn page_articles(
    uri: http::Uri,
    Extension(auth): Extension<AuthContext>,
    Extension(articles_repo): Extension<ArticlesRepo>,
) -> impl IntoResponse {
    render_with_meta(
        || {
            provide_context(uri);
            provide_context(auth);
            provide_context(articles_repo);
        },
        || async {
            html! {
                <Layout title="Articles - Antonio Pitasi" description="Antonio's articles on various topics related to software engineering and technology.">
                    <Articles>""</Articles>
                </Layout>
            }
        }
    ).await
}

pub async fn articles_rss(Extension(articles_repo): Extension<ArticlesRepo>) -> impl IntoResponse {
    let articles = articles_repo.list();
    let last_modified = articles.first().unwrap().datetime.clone();

    let entries = articles
        .iter()
        .take(10)
        .map(|article| {
            let url = format!("https://anto.pt/articles/{}", article.slug);
            EntryBuilder::default()
                .title(TextBuilder::default().value(article.title.clone()).build())
                .id(url.clone())
                .links(vec![LinkBuilder::default()
                    .rel("alternate".to_string())
                    .mime_type(Some("text/html".to_string()))
                    .href(url)
                    .build()])
                .published(Some(article.datetime.clone()))
                .updated(article.datetime.clone())
                .content(Some(
                    ContentBuilder::default()
                        .content_type(Some("html".to_string()))
                        .value(Some(article.content.clone()))
                        .build(),
                ))
                .build()
        })
        .collect::<Vec<_>>();

    let feed = FeedBuilder::default()
        .title("Antonio Pitasi's Articles")
        .id("https://anto.pt/articles".to_string())
        .links(vec![
            LinkBuilder::default()
                .rel("self".to_string())
                .href("https://anto.pt/articles/atom.xml".to_string())
                .build(),
            LinkBuilder::default()
                .href("https://anto.pt/articles".to_string())
                .build(),
        ])
        .authors(vec![PersonBuilder::default()
            .name("Antonio Pitasi".to_string())
            .build()])
        .updated(last_modified)
        .entries(entries)
        .build();

    feed.to_string()
}

#[props]
pub struct ArticlesProps {
    children: String,
}

#[component]
pub fn Articles(props: ArticlesProps) -> String {
    let repo = expect_context::<ArticlesRepo>();
    let v = repo
        .list()
        .into_iter()
        .map(|article| async move {
            let href = format!("/articles/{}", article.slug);
            let title = &article.title;
            let c = html! {
                <div class="flex flex-col">
                    <span class="font-semibold">
                        {title}
                    </span>
                    <span class="opacity-60">
                        {article.datetime.format("%B %d, %Y").to_string()}
                    </span>
                </div>
            };

            html! {
                <SidebarNavItem href=href>
                    {c}
                </SidebarNavItem>
            }
        })
        .collect_fragment_async()
        .await;

    let children = if !props.children.is_empty() {
        html! {
            <div class="absolute inset-0 lg:static empty:hidden">
                {props.children}
            </div>
        }
    } else {
        html! {}
    };

    html! {
        <Dedup id="atom".to_string()>
            <link rel="alternate" type="application/atom+xml" title="RSS Feed for anto.pt articles" href="/articles/atom.xml" />
        </Dedup>
        <div class="relative h-full w-full flex-row lg:grid lg:grid-cols-[20rem_minmax(0,1fr)]">
            <SecondarySidebar title="Articles".into()>
                {v}
            </SecondarySidebar>
            {children}
        </div>
    }
}

pub async fn page_article(
    uri: http::Uri,
    Extension(auth): Extension<AuthContext>,
    Extension(articles_repo): Extension<ArticlesRepo>,
    Path(slug): Path<String>,
) -> Response {
    let article = articles_repo.get_article_by_slug(&slug);
    if article.is_none() {
        return (http::StatusCode::NOT_FOUND, "404 not found").into_response();
    }
    let article = article.unwrap().clone();
    let title = format!("{} - Antonio Pitasi", article.title.clone());
    let og_image = format!(
        "https://anto.pt/articles/{}/social-image.png",
        article.slug.clone()
    );

    render_with_meta(|| {
        provide_context(uri);
        provide_context(auth);
        provide_context(articles_repo);
    }, || async {
        html! {
            <Layout title=title description="Antonio's articles on various topics related to software engineering and technology." head=html!{
                <MetaOGImage content=og_image />
            }>
                <Articles>
                    <ArticleContent a=article />
                </Articles>
            </Layout>
        }
    })
    .await
    .into_response()
}

#[props]
pub struct ArticleContentProps {
    a: Article,
}

#[component]
fn ArticleContent(props: ArticleContentProps) -> String {
    html! {
        <main class="typography relative min-h-full bg-floralwhite pb-24 lg:pb-0">
            <Header title=props.a.title.clone() />
            <article class="w-full bg-floralwhite p-8">
                <div class="mx-auto max-w-2xl">
                    <div class="flex flex-col gap-3">
                        <h1 class="title font-neu font-semibold text-darkviolet text-4xl">
                            {props.a.title}
                        </h1>
                        <div class="flex flex-row">
                            <span class="text-gray-500">
                                "Written on " {props.a.datetime.format("%B %d, %Y").to_string()} "."
                            </span>
                        </div>
                    </div>
                    <div class="mt-4">
                        {props.a.content}
                    </div>
                </div>
            </article>
        </main>
    }
}
