use axum::{extract::Path, http, response::IntoResponse, Extension};
use sycamore::prelude::*;

use crate::{
    apps::{App, AppsRepo},
    components::layout::{Header, Layout, SecondarySidebar, SidebarNavItem},
    icons::{Heart, Link},
    images::Image,
    root,
    sycamore::Title,
};

use super::auth::AuthContext;

pub async fn handler(
    uri: http::Uri,
    Extension(auth): Extension<AuthContext>,
    Extension(apps_repo): Extension<AppsRepo>,
) -> impl IntoResponse {
    root! {
        (uri, auth, apps_repo),
        Title { "Apps - Antonio Pitasi" }
        Layout {
            Apps {}
        }
    }
}

pub async fn handler_app(
    uri: http::Uri,
    Extension(auth): Extension<AuthContext>,
    Extension(apps_repo): Extension<AppsRepo>,
    Path(slug): Path<String>,
) -> impl IntoResponse {
    let app = apps_repo.get_by_slug(&slug).await.unwrap();
    root! {
        (uri, auth, apps_repo),
        Title { "Apps - Antonio Pitasi" }
        Layout {
            Apps {
                AppContent(app=app)
            }
        }
    }
}

#[derive(Props)]
pub struct AppsProps<'a, G: Html> {
    children: Option<Children<'a, G>>,
}

#[component]
pub async fn Apps<'a, G: Html>(cx: Scope<'a>, props: AppsProps<'a, G>) -> View<G> {
    let repo = use_context::<AppsRepo>(cx);
    let apps = repo.list().await;

    let v = View::new_fragment(
        apps.into_iter()
            .map(|app| {
                let href = format!("/apps/{}", app.slug);
                let name = app.name.clone();
                let c = view! {
                    cx,
                    div(class="flex flex-row gap-4 items-center") {
                        Image(sources=app.images, alt=app.name.clone(), class="w-10 h-10 rounded-lg bg-white object-contain".into())
                        span(class="font-semibold"){ (name) }
                    }
                };

                view! { cx, SidebarNavItem(href={href}){(c)} }
            })
            .collect(),
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
            SecondarySidebar(title="Apps".into()) {(v)}
            (children)
        }
    }
}

#[derive(Props)]
pub struct AppContentProps {
    app: App,
}

#[component]
fn AppContent<G: Html>(cx: Scope, props: AppContentProps) -> View<G> {
    let title = props.app.name.clone();
    let title2 = props.app.name.clone();

    view! { cx,
        main(class="relative min-h-full bg-floralwhite pb-24 lg:pb-0") {
            Header(title=title) {}
            article(class="w-full bg-floralwhite p-8") {
                div(class="mx-auto max-w-2xl space-y-6") {
                    div(class="flex flex-row gap-8 items-center") {
                        Image(sources=props.app.images.clone(), alt=title2, class="w-20 h-20 rounded-2xl bg-white object-contain".into())

                        h1(class="title font-neu font-semibold text-darkviolet text-4xl") {
                            (props.app.name)
                        }
                    }

                    div(class="typography", dangerously_set_inner_html=props.app.description.clone()) { }

                    a(class="block flex flex-row justify-center items-center gap-2 w-full bg-darkviolet text-white py-2 px-4 rounded-lg shadow-md hover:bg-darkviolet-light", href=props.app.url, target="_blank") {
                        Link {}
                        span{ "Visit" }
                    }

                    div(class="text-gray-500") {
                        p { "Do you use this app too? Click the " Heart(filled=true, class="inline opacity-70".into()) " button!" }
                    }
                }
            }
        }
    }
}
