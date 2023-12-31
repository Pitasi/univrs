use futures::future::join_all;
use rscx::html;
use sqlx::{FromRow, PgPool};

use crate::{images::ImageSrc, markdown::Markdown};

#[derive(Debug, FromRow)]
pub struct AppRow {
    pub slug: String,
    pub name: String,
    pub description: String,
    pub url: String,
    pub images: Vec<String>,
}

#[derive(Debug)]
pub struct App {
    pub slug: String,
    pub name: String,
    pub description: String,
    pub url: String,
    pub images: Vec<ImageSrc>,
}

impl App {
    async fn from_row(row: AppRow) -> Self {
        Self {
            slug: row.slug,
            name: row.name,
            description: html! {
                <Markdown source=row.description />
            },
            url: row.url,
            images: row.images.into_iter().map(From::from).collect(),
        }
    }
}

#[derive(Clone)]
pub struct AppsRepo {
    pool: PgPool,
}

impl AppsRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn list(&self) -> Vec<App> {
        let mut conn = self.pool.acquire().await.unwrap();
        let res = sqlx::query_as::<_, AppRow>(
            r#"
            select * from apps
            order by name asc
        "#,
        )
        .fetch_all(&mut conn)
        .await
        .unwrap();

        join_all(
            res.into_iter()
                .map(|row| async { App::from_row(row).await }),
        )
        .await
    }

    pub async fn get_by_slug(&self, slug: &str) -> Option<App> {
        let mut conn = self.pool.acquire().await.unwrap();
        let res = sqlx::query_as::<_, AppRow>(
            r#"
            select * from apps
            where slug = $1
        "#,
        )
        .bind(slug)
        .fetch_optional(&mut conn)
        .await
        .unwrap();
        App::from_row(res?).await.into()
    }
}
