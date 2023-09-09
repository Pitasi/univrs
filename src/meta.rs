use std::{collections::HashMap, sync::Arc};

use rscx::{
    component,
    context::{expect_context, provide_context},
    props,
};
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct MetaContext {
    pub dedup_elements: Arc<Mutex<HashMap<String, String>>>,
}

impl MetaContext {
    pub fn new() -> Self {
        Self {
            dedup_elements: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn register(&mut self, key: String, value: String) {
        self.dedup_elements
            .clone()
            .lock_owned()
            .await
            .insert(key, value);
    }
}

#[props]
pub struct DedupProps {
    pub id: String,
    pub children: String,
}

#[component]
pub async fn Dedup(props: DedupProps) -> String {
    let mut ctx = expect_context::<MetaContext>();
    ctx.register(props.id, props.children).await;
    "".into()
}

#[component]
pub async fn MetaContextRender() -> String {
    let ctx = expect_context::<MetaContext>();
    let mut dedup_elements = ctx.dedup_elements.lock_owned().await;
    let mut dedup_elements = dedup_elements.drain().collect::<Vec<_>>();
    dedup_elements.sort_by(|a, b| a.0.cmp(&b.0));
    let dedup_elements = dedup_elements
        .into_iter()
        .map(|(_, v)| v)
        .collect::<Vec<_>>();
    dedup_elements.join("")
}

pub async fn render_with_meta<F>(
    pre_fn: impl FnOnce() -> () + Send + 'static,
    render_fn: impl FnOnce() -> F + Send + 'static,
) -> axum::response::Html<String>
where
    F: futures::Future<Output = String> + Send + 'static,
{
    rscx::axum::render(async move {
        provide_context(MetaContext::new());
        pre_fn();
        render_fn().await
    })
    .await
}
