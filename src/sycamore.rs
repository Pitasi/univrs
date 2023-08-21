use std::{cell::RefCell, collections::HashMap, rc::Rc, thread::available_parallelism};

use once_cell::sync::OnceCell;
use sycamore::{prelude::*, render_to_string, render_to_string_await_suspense};
use tokio_util::task::LocalPoolHandle;

fn get_render_pool() -> LocalPoolHandle {
    static LOCAL_POOL: OnceCell<LocalPoolHandle> = OnceCell::new();
    LOCAL_POOL
        .get_or_init(|| {
            tokio_util::task::LocalPoolHandle::new(
                available_parallelism().map(Into::into).unwrap_or(1),
            )
        })
        .clone()
}

pub fn shell(html_attrs: String, body_attrs: String, head: String, body: String) -> String {
    format!("<!DOCTYPE html><html {html_attrs}><head>{head}</head><body {body_attrs}>{body}</body></html>")
}

pub struct HeadCtx<G: Html> {
    head_els: Vec<View<G>>,
    deduped_head_els: HashMap<String, View<G>>,
    html_attrs: Vec<String>,
    body_attrs: Vec<String>,
}

impl<G: Html> HeadCtx<G> {
    fn render_head(&self) -> Vec<View<G>> {
        self.deduped_head_els
            .clone()
            .into_values()
            .map(|v| v)
            .chain(self.head_els.clone().into_iter())
            .collect()
    }
}

pub async fn render(
    view: impl FnOnce(Scope<'_>) -> View<SsrNode> + 'static + std::marker::Send,
) -> axum::response::Html<String> {
    let pool = get_render_pool();
    let out = pool.spawn_pinned(|| async {
        let meta = HeadCtx {
            head_els: vec![],
            deduped_head_els: HashMap::new(),
            body_attrs: vec![],
            html_attrs: vec![],
        };
        let meta_ref = Rc::new(RefCell::new(meta));
        let meta_ref2 = meta_ref.clone();
        let meta_ref3 = meta_ref.clone();

        // Render inner app, waiting for async components
        let body = render_to_string_await_suspense(|cx| {
            provide_context(cx, meta_ref2);
            view(cx)
        })
        .await;

        // Render shell
        let head = render_to_string(
            |cx| view! { cx, (View::new_fragment(meta_ref.borrow().render_head())) },
        );
        let html_attrs = meta_ref3.borrow().html_attrs.join(" ");
        let body_attrs = meta_ref3.borrow().body_attrs.join(" ");

        let res = shell(html_attrs, body_attrs, head, body);
        res
    });

    let ui = out.await.unwrap();

    axum::response::Html(ui)
}

#[macro_export]
macro_rules! root {
    (($($es:expr),+), $($tokens:tt)*) => {{
        let h = crate::sycamore::render(move |cx| {
            $(provide_context(cx, $es);)*
            view!(cx, $($tokens)*)
        }).await;
        h
    }};

    ($($tokens:tt)*) => {{
        let h = crate::sycamore::render(move |cx| {
            view!(cx, $($tokens)*)
        }).await;
        h
    }};
}

#[derive(Props)]
pub struct HeadProps<'a, G: Html> {
    children: Children<'a, G>,
}

#[component]
pub fn Head<'a, G: Html>(cx: Scope<'a>, props: HeadProps<'a, G>) -> View<G> {
    let m = use_context::<Rc<RefCell<HeadCtx<G>>>>(cx);
    let children = props.children.call(cx);
    m.borrow_mut().head_els.push(children);
    view! { cx, }
}

#[derive(Props)]
pub struct DedupProps<'a, G: Html> {
    key: String,
    children: Children<'a, G>,
}

#[component]
pub fn Dedup<'a, G: Html>(cx: Scope<'a>, props: DedupProps<'a, G>) -> View<G> {
    let m = use_context::<Rc<RefCell<HeadCtx<G>>>>(cx);
    let children = props.children.call(cx);
    if m.borrow().deduped_head_els.contains_key(&props.key) {
        // element already exists, don't overwrite it.
        // this works because children are typically rendered before parents in Sycamore
        return view! { cx, };
    }
    m.borrow_mut().deduped_head_els.insert(props.key, children);
    view! { cx, }
}

#[derive(Props)]
pub struct TitleProps<'a, G: Html> {
    children: Children<'a, G>,
}

#[component]
pub fn Title<'a, G: Html>(cx: Scope<'a>, props: TitleProps<'a, G>) -> View<G> {
    let children = props.children.call(cx);
    view! { cx, Dedup(key="title".into()){
        title { (children) }
    }}
}

#[derive(Props)]
pub struct MetatagProps<'a, G: Html> {
    name: String,
    attributes: Attributes<'a, G>,
}

#[component]
pub fn Metatag<'a, G: Html>(cx: Scope<'a>, props: MetatagProps<'a, G>) -> View<G> {
    view! { cx, Dedup(key=props.name.clone()) {
        meta(..props.attributes, name=props.name) {}
    } }
}

#[derive(Props)]
pub struct BodyProps<'a, G: Html> {
    attributes: Attributes<'a, G>,
}

#[component]
pub fn Body<'a, G: Html>(cx: Scope<'a>, props: BodyProps<'a, G>) -> View<G> {
    let m = use_context::<Rc<RefCell<HeadCtx<G>>>>(cx);
    let attrs = attributes_to_string(props.attributes);
    m.borrow_mut().body_attrs.push(attrs);
    view! { cx, }
}

#[derive(Props)]
pub struct HtmlProps<'a, G: Html> {
    attributes: Attributes<'a, G>,
}

#[component]
pub fn Html<'a, G: Html>(cx: Scope<'a>, props: HtmlProps<'a, G>) -> View<G> {
    let m = use_context::<Rc<RefCell<HeadCtx<G>>>>(cx);
    let attrs = attributes_to_string(props.attributes);
    m.borrow_mut().html_attrs.push(attrs);
    view! { cx, }
}

fn attributes_to_string<G: GenericNode>(attrs: Attributes<G>) -> String {
    attrs
        .drain()
        .into_iter()
        .filter_map(|(k, v)| match v {
            AttributeValue::Str(s) => Some(format!(r#"{}="{}""#, k, s)),
            AttributeValue::DynamicStr(mut f) => Some(format!(r#"{}="{}""#, k, f())),
            AttributeValue::Bool(b) if b => Some(format!("{}", k)),
            AttributeValue::DynamicBool(mut f) => {
                if f() {
                    Some(format!("{}", k))
                } else {
                    None
                }
            }
            _ => None,
        })
        .collect::<Vec<_>>()
        .join(" ")
}

#[macro_export]
macro_rules! xstyledview {
    ($cx:expr, $s:expr, $($tokens:tt)*) => {{
        let styles = $s.unwrap();
        let css = styles.get_style_str().to_string();
        let class = styles.get_class_name().to_owned();
        let class2 = class.clone();
        let cx = $cx;

        let head = view!(cx,
            Dedup(key=class.clone()) {
                style {
                    (css)
                }
            }
        );

        let node: View<G> = view!(cx, $($tokens)*);
        node.as_node()
            .expect("Tried to apply styledview! macro to a non-node. Wrap your component in a <div> or another element, this is needed to apply the `class` attribute to something.")
            .set_attribute("class".into(), class2.into());

        View::new_fragment(vec![head, node])
    }};
}
