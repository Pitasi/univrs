use std::rc::Rc;
use std::sync::Mutex;

use axum::response::{Html, IntoResponse};
use leptos::{use_context, IntoView, Scope};
use leptos_meta::{provide_meta_context, MetaContext};

/// Render a Leptos component into a HTML string.
#[derive(Debug, Default, Clone)]
pub struct LeptosComponent<F, N>
where
    F: FnOnce(Scope) -> N + 'static,
    N: IntoView,
{
    pub f: F,
}

impl<F, N> IntoResponse for LeptosComponent<F, N>
where
    F: FnOnce(Scope) -> N + 'static,
    N: IntoView,
{
    fn into_response(self) -> axum::response::Response {
        let content = leptos::ssr::render_to_string(self.f);
        Html(content).into_response()
    }
}

/// Render a Leptos component into a HTML document. It will also render the
/// `<head>` and `<body>` tags.
/// Example:
/// ```
/// pub async fn axum_handler() -> impl IntoResponse {
///     Leptos(|cx| {
///         view! {
///             cx,
///             <div>
///                 <h1>"Hello, world!"</h1>
///             </div>
///         }
///     })
/// }
/// ```
#[derive(Debug, Default, Clone)]
pub struct Leptos<F, N>
where
    F: FnOnce(Scope) -> N + 'static,
    N: IntoView,
{
    pub f: F,
}

impl<F, N> IntoResponse for Leptos<F, N>
where
    F: FnOnce(Scope) -> N + 'static,
    N: IntoView,
{
    fn into_response(self) -> axum::response::Response {
        let head: Rc<Mutex<String>> = Default::default();
        let body: Rc<Mutex<String>> = Default::default();
        let html: Rc<Mutex<String>> = Default::default();
        let head2 = head.clone();
        let body2 = body.clone();
        let html2 = html.clone();

        let content = leptos::ssr::render_to_string(move |cx| {
            provide_meta_context(cx);

            let v = (self.f)(cx);

            let meta = use_context::<MetaContext>(cx);
            let head_str = meta
                .as_ref()
                .map(|meta| meta.dehydrate())
                .unwrap_or_default();
            let body_str = meta
                .as_ref()
                .and_then(|meta| meta.body.as_string())
                .unwrap_or_default();
            let html_str = meta
                .as_ref()
                .and_then(|meta| meta.html.as_string())
                .unwrap_or_default();

            head.clone().try_lock().unwrap().push_str(&head_str);
            body.clone().try_lock().unwrap().push_str(&body_str);
            html.clone().try_lock().unwrap().push_str(&html_str);

            v
        });

        let content = format!(
            "<!DOCTYPE html><html{}><head>{}</head><body{}>{}</body></html>",
            html2.try_lock().unwrap(),
            head2.try_lock().unwrap(),
            body2.try_lock().unwrap(),
            content,
        );

        Html(content).into_response()
    }
}

#[macro_export]
macro_rules! leptos_root {
    (($($es:expr),+), $($tokens:tt)*) => {{
        Leptos{
            f: move |cx| {
                $(provide_context(cx, $es);)*
                view! {cx, $($tokens)*}
            },
        }
    }};

    ($($tokens:tt)*) => {{
        Leptos{
            f: move |cx| {
                view! {cx, $($tokens)*}
            },
        }
    }};
}

#[macro_export]
macro_rules! leptos_component {
    (($($es:expr),+), $($tokens:tt)*) => {{
        LeptosComponent{
            f: move |cx| {
                $(provide_context(cx, $es);)*
                view! {cx, $($tokens)*}
            },
        }
    }};

    ($($tokens:tt)*) => {{
        LeptosComponent{
            f: move |cx| {
                view! {cx, $($tokens)*}
            },
        }
    }};
}

#[macro_export]
macro_rules! styledview {
    ($cx:expr, $s:expr, $($tokens:tt)*) => {{
        let styles = $s.unwrap();
        let css = styles.get_style_str().to_string();
        let class = styles.get_class_name();
        let cx = $cx;
        view!(cx, class=class,
            <Style id=class.to_owned()>{css}</Style>
            $($tokens)*
        )
    }};
}
