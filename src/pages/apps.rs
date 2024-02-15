use axum::{extract::Path, http, response::IntoResponse, Extension};
use rscx::{
    component,
    context::{expect_context, provide_context},
    html, props, CollectFragmentAsync,
};

use crate::{
    apps::{App, AppsRepo},
    components::layout::{Header, Layout, SecondarySidebar, SidebarNavItem},
    icons::{Heart, Link},
    images::Image,
    meta::render_with_meta,
};

use super::auth::AuthContext;

pub async fn handler(
    uri: http::Uri,
    Extension(auth): Extension<AuthContext>,
    Extension(apps_repo): Extension<AppsRepo>,
) -> impl IntoResponse {
    render_with_meta(
        || {
            provide_context(uri);
            provide_context(auth);
            provide_context(apps_repo);
        },
        || async {
            html! {
                <Layout title="Uses - Antonio Pitasi">
                    <Apps />
                </Layout>
            }
        },
    )
    .await
}

pub async fn handler_app(
    uri: http::Uri,
    Extension(auth): Extension<AuthContext>,
    Extension(apps_repo): Extension<AppsRepo>,
    Path(slug): Path<String>,
) -> impl IntoResponse {
    let app = apps_repo.get_by_slug(&slug).await.unwrap();
    let name = app.name.clone();
    render_with_meta(
        || {
            provide_context(uri);
            provide_context(auth);
            provide_context(apps_repo);
        },
        || async move {
            html! {
                <Layout title=format!("{name} - Uses - Antonio Pitasi")>
                    <Apps>
                        <AppContent app=app />
                    </Apps>
                </Layout>
            }
        },
    )
    .await
}

#[props]
pub struct AppsProps {
    #[builder(default)]
    children: String,
}

#[component]
pub async fn Apps(props: AppsProps) -> String {
    let repo = expect_context::<AppsRepo>();
    let apps = repo.list().await;

    let v = apps.into_iter()
            .map(|app| async move {
                let href = format!("/uses/{}", app.slug);
                let name = app.name.clone();
                let c = html! {
                    <div class="flex flex-row gap-4 items-center">
                        <Image sources=app.images alt=app.name.clone() class="w-10 h-10 rounded-lg bg-white object-contain".into() />
                        <span class="font-semibold">
                            {name}
                        </span>
                    </div>
                };

                html! {
                    <SidebarNavItem href=href>
                        {c}
                    </SidebarNavItem>
                }
            })
            .collect_fragment_async().await;

    html! {
        <div class="relative h-full w-full flex-row lg:grid lg:grid-cols-[20rem_minmax(0,1fr)]">
            <SecondarySidebar title="Uses".into()>
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
pub struct AppContentProps {
    app: App,
}

#[component]
fn AppContent(props: AppContentProps) -> String {
    let title = props.app.name.clone();
    let title2 = props.app.name.clone();

    html! {
        <main class="relative min-h-full bg-floralwhite pb-24 lg:pb-0">
            <Header title=title />
            <article class="w-full bg-floralwhite p-8">
                <div class="mx-auto max-w-2xl space-y-6">
                    <div class="flex flex-row gap-4 sm:gap-8 items-center">
                        <Image sources=props.app.images.clone() alt=title2 class="w-20 h-20 rounded-2xl bg-white object-contain".into() />
                        <h1 class="title font-neu font-semibold text-darkviolet text-3xl md:text-4xl">
                            {props.app.name}
                        </h1>
                    </div>

                    <div class="typography">
                        {props.app.description}
                    </div>

                    <a class="block flex flex-row justify-center items-center gap-2 w-full bg-darkviolet text-white py-2 px-4 rounded-lg shadow-md hover:bg-darkviolet-light"
                        href=props.app.url target="_blank">
                        <Link />
                        <span>Visit</span>
                    </a>

                    <div class="text-gray-500">
                        <p>
                            "Do you use this app too? Click the "
                            <Heart filled=true class=Some("inline opacity-70".into()) />
                            " button!"
                        </p>
                    </div>
                </div>
            </article>
        </main>
    }
}
