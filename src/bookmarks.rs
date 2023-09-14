use chrono::DateTime;
use futures::future::join_all;
use reqwest::Url;
use sqlx::{postgres::PgQueryResult, Executor, FromRow, PgPool};

#[derive(Debug, FromRow)]
pub struct BookmarkRow {
    pub slug: String,
    pub url: String,
    pub title: String,
    pub description: String,
    pub favicon: Option<String>,
    pub image: Option<String>,
    pub posted_at: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug)]
pub struct Bookmark {
    pub slug: String,
    pub url: String,
    pub hostname: String,
    pub title: String,
    pub description: String,
    pub favicon: Option<String>,
    pub image: Option<String>,
    pub posted_at: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl Bookmark {
    async fn from_row(row: BookmarkRow) -> Self {
        let hostname = Url::parse(&row.url)
            .unwrap()
            .host_str()
            .unwrap()
            .to_string();
        Self {
            slug: row.slug,
            url: row.url,
            hostname,
            title: row.title,
            description: row.description,
            favicon: row.favicon,
            image: row.image,
            posted_at: row.posted_at,
            created_at: row.created_at,
        }
    }
}

#[derive(Clone)]
pub struct BookmarksRepo {
    pool: PgPool,
}

impl BookmarksRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn list(&self) -> Vec<Bookmark> {
        let mut conn = self.pool.acquire().await.unwrap();
        let res = sqlx::query_as::<_, BookmarkRow>(
            r#"
            select * from bookmarks
            order by created_at desc
        "#,
        )
        .fetch_all(&mut conn)
        .await
        .unwrap();

        join_all(
            res.into_iter()
                .map(|row| async { Bookmark::from_row(row).await }),
        )
        .await
    }

    pub async fn get_by_slug(&self, slug: &str) -> Option<Bookmark> {
        let mut conn = self.pool.acquire().await.unwrap();
        let res = sqlx::query_as::<_, BookmarkRow>(
            r#"
            select * from bookmarks
            where slug = $1
        "#,
        )
        .bind(slug)
        .fetch_optional(&mut conn)
        .await
        .unwrap();
        Bookmark::from_row(res?).await.into()
    }

    pub async fn add(
        &self,
        slug: &str,
        url: &str,
        title: &str,
        description: &str,
        favicon: Option<&str>,
        image: Option<&str>,
        posted_at: &DateTime<chrono::Utc>,
    ) -> Result<PgQueryResult, sqlx::Error> {
        let mut conn = self.pool.acquire().await.unwrap();
        conn.execute(
            sqlx::query(
                r#"
            insert into bookmarks (slug, url, title, description, favicon, image, posted_at)
            values ($1, $2, $3, $4, $5, $6, $7)
            "#,
            )
            .bind(slug)
            .bind(url)
            .bind(title)
            .bind(description)
            .bind(favicon)
            .bind(image)
            .bind(posted_at),
        )
        .await
    }
}
