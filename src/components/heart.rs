use axum::{
    extract::Query,
    http::{self, HeaderMap},
    response::IntoResponse,
    Extension, Form,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use sycamore::prelude::*;

use crate::{
    icons::Heart,
    pages::auth::{AuthContext, User},
    root,
};

#[derive(Serialize, Deserialize)]
pub struct LikeBtnPayload {
    pub url: String,
}

async fn theasyncwrapper(
    pool: PgPool,
    user: Option<User>,
    url: &str,
    act: bool,
) -> (i64, bool, String) {
    let mut conn = pool.acquire().await.unwrap();
    let mut has_like = match &user {
        Some(u) => sqlx::query_as::<_, Like>(
            r#"
            select * from likes
            where user_id = $1
            and url = $2
        "#,
        )
        .bind(u.id)
        .bind(url)
        .fetch_one(&mut conn)
        .await
        .is_ok(),
        None => false,
    };

    if act && user.is_some() {
        let u = user.unwrap();
        if has_like {
            sqlx::query(
                r#"
                    delete from likes
                    where user_id = $1
                    and url = $2
                "#,
            )
            .bind(u.id)
            .bind(url)
            .execute(&mut conn)
            .await
            .unwrap();
            has_like = false;
        } else {
            sqlx::query(
                r#"
                    insert into likes (user_id, url)
                    values ($1, $2)
                "#,
            )
            .bind(u.id)
            .bind(url)
            .execute(&mut conn)
            .await
            .unwrap();
            has_like = true;
        }
    }

    let count: i64 = sqlx::query_scalar(
        r#"
            select count(*) from likes
            where url = $1
        "#,
    )
    .bind(url)
    .fetch_one(&pool)
    .await
    .unwrap();

    let payload = serde_json::to_string(&LikeBtnPayload {
        url: url.to_string(),
    })
    .unwrap();

    (count, has_like, payload)
}

#[derive(Props)]
pub struct HeartButtonProps {
    pub count: String,
    pub has_like: bool,
    pub payload: String,
}

#[component]
fn HeartButton<G: Html>(cx: Scope, props: HeartButtonProps) -> View<G> {
    view! { cx,
        button(
            class="inline-flex items-center justify-center text-sm font-medium transition-colors focus:outline-none focus:ring-2 focus:ring-offset0 disabled:opacity-50 disabled:pointer-events-none bg-transparent hover:bg-slate-100 data-[state=open]:bg-transparent h-9 px-2 rounded-md",
            hx-post="/components/like-btn",
            hx-trigger="click",
            hx-target="this",
            hx-swap="outerHTML",
            hx-vals=props.payload,
            data-loading-disable=true) {
                div(class="flex flex-row items-center justify-center gap-2 font-neu text-3xl font-semibold") {
                    Heart(filled=props.has_like)
                    span(class="translate-y-0.5") { (props.count) }
                }
        }
    }
}

#[derive(Debug, sqlx::FromRow)]
pub struct Like {
    pub id: i64,
    pub user_id: i64,
    pub url: String,
}

#[derive(Deserialize)]
pub struct PageLikeBtnQuery {
    url: String,
}

pub async fn handler_get(
    auth: AuthContext,
    Extension(pool): Extension<PgPool>,
    query: Query<PageLikeBtnQuery>,
) -> impl IntoResponse {
    let (count, has_like, payload) =
        theasyncwrapper(pool, auth.current_user, &query.url, false).await;
    root! {
        HeartButton(count=count.to_string(), has_like=has_like, payload=payload)
    }
}

pub async fn handler_post(
    auth: AuthContext,
    Extension(pool): Extension<PgPool>,
    Form(payload): Form<LikeBtnPayload>,
) -> impl IntoResponse {
    let mut header_map = HeaderMap::new();
    if auth.current_user.is_none() {
        let redirect = format!("/auth/login?redirect_to={}", payload.url);
        header_map.insert("HX-Redirect", redirect.parse().unwrap());
    }

    let (count, has_like, payload) =
        theasyncwrapper(pool, auth.current_user, &payload.url, true).await;

    (
        header_map,
        root! {
            HeartButton(count=count.to_string(), has_like=has_like, payload=payload)
        },
    )
}
