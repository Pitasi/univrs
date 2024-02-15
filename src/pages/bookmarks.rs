use axum::{extract::Path, http, response::IntoResponse, Extension};
use rscx::{
    component,
    context::{expect_context, provide_context},
    html, props, CollectFragmentAsync,
};

use crate::{
    bookmarks::{Bookmark, BookmarksRepo},
    components::layout::{Header, Layout, SecondarySidebar, SidebarNavItem},
    icons::Link,
    meta::render_with_meta,
};

use super::auth::AuthContext;

pub async fn handler(
    uri: http::Uri,
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
                    <Bookmarks />
                </Layout>
            }
        },
    )
    .await
}

pub async fn handler_bookmark(
    uri: http::Uri,
    Extension(auth): Extension<AuthContext>,
    Extension(bookmarks_repo): Extension<BookmarksRepo>,
    Path(slug): Path<String>,
) -> impl IntoResponse {
    let bookmark = bookmarks_repo.get_by_slug(&slug).await.unwrap();
    let title = bookmark.title.clone();
    render_with_meta(
        || {
            provide_context(uri);
            provide_context(auth);
            provide_context(bookmarks_repo);
        },
        || async move {
            html! {
                <Layout title=format!("{title} - Bookmarks - Antonio Pitasi")>
                    <Bookmarks>
                        <BookmarkContent bookmark=bookmark />
                    </Bookmarks>
                </Layout>
            }
        },
    )
    .await
}

#[props]
pub struct BookmarksProps {
    #[builder(default)]
    children: String,
}

#[component]
pub async fn Bookmarks(props: BookmarksProps) -> String {
    let repo = expect_context::<BookmarksRepo>();
    let bookmarks = repo.list().await;

    let v = bookmarks
        .into_iter()
        .map(|bookmark| async move {
            let href = format!("/bookmarks/{}", bookmark.slug);
            let title = bookmark.title.clone();
            let c = html! {
                <div class="flex flex-col gap-1">
                    <span class="font-semibold text-balance">
                        {title}
                    </span>
                    <span class="flex flex-row opacity-60 items-center text-sm">
                        {
                            bookmark.favicon.map_or(
                                html! {
                                    <div class="block rounded-sm bg-gray-500 w-4 h-4 mr-1 shrink-0" />
                                },
                                |favicon| html! {
                                    <img src=favicon alt="Favicon" class="block rounded-sm w-4 h-4 mr-1" />
                                },
                            )
                        }
                        {&bookmark.hostname}
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

    html! {
        <div class="relative h-full w-full flex-row lg:grid lg:grid-cols-[20rem_minmax(0,1fr)]">
            <SecondarySidebar title="Bookmarks".into()>
                {v}
            </SecondarySidebar>
            { if !props.children.is_empty() {
                html! {
                    <div class="absolute inset-0 lg:static empty:hidden">
                        {props.children}
                    </div>
                }
            } else { html!{} } }
        </div>
    }
}

#[props]
pub struct BookmarkContentProps {
    bookmark: Bookmark,
}

#[component]
fn BookmarkContent(props: BookmarkContentProps) -> String {
    let title = props.bookmark.title.clone();

    html! {
        <main class="relative min-h-full bg-floralwhite pb-24 lg:pb-0">
            <Header title=title />
            <article class="w-full bg-floralwhite p-8">
                <div class="mx-auto max-w-2xl space-y-6">
                    <div class="flex flex-col gap-4 sm:gap-8">
                        { props.bookmark.image.map(|src| html! {
                            <img src=src alt=&props.bookmark.title class="w-full border" />
                        }).unwrap_or(html!{}) }

                        <h1 class="title font-neu font-semibold text-darkviolet text-3xl md:text-4xl">
                            {props.bookmark.title}
                        </h1>

                        <a href=&props.bookmark.url target="_blank">
                            <p class="flex flex-row opacity-60 items-center text-sm">
                                {
                                    props.bookmark.favicon.map_or(
                                        html! {
                                            <span class="rounded-sm bg-gray-500 w-6 h-6 mr-2 shrink-0" />
                                        },
                                        |favicon| html! {
                                            <img src=favicon alt="Favicon" class="block rounded-sm w-6 h-6 mr-2" />
                                        },
                                    )
                                }
                                <span>{&props.bookmark.hostname}</span>
                            </p>
                        </a>

                        <p class="opacity-60">
                            Written on {&props.bookmark.posted_at.format("%B %e, %Y").to_string()}.
                            <br />
                            Bookmarked on {&props.bookmark.created_at.format("%B %e, %Y").to_string()}.
                        </p>
                    </div>

                    <div class="typography">
                        {props.bookmark.description}
                    </div>

                    <a class="block flex flex-row justify-center items-center gap-2 w-full bg-darkviolet text-white py-2 px-4 rounded-lg shadow-md hover:bg-darkviolet-light"
                        href=props.bookmark.url target="_blank">
                        <Link />
                        <span>Visit</span>
                    </a>
                </div>
            </article>
        </main>
    }
}
