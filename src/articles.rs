use chrono::{DateTime, FixedOffset};
use rscx::html;

use crate::markdown::{load_dir, Markdown};

#[derive(Clone, Debug)]
pub struct Article {
    pub title: String,
    pub datetime: DateTime<FixedOffset>,
    pub slug: String,
    pub content: String,
    pub unlisted: bool,
}

#[derive(Clone, Debug)]
pub struct ArticlesRepo {
    pub articles: Vec<Article>,
}

impl<'a> ArticlesRepo {
    pub async fn new() -> Self {
        let markdown_files = load_dir("./articles");
        let mut articles = vec![];
        for md in markdown_files {
            let title = md.frontmatter["title"].as_str().unwrap().to_string();
            let datetime_str = md.frontmatter["datetime"].as_str().unwrap();
            let datetime = DateTime::parse_from_rfc3339(datetime_str).unwrap();
            let unlisted = md.frontmatter["unlisted"].as_bool().unwrap_or(false);
            articles.push(Article {
                title,
                datetime,
                slug: md.name.clone(),
                content: async {
                    html! {
                        <Markdown source=md.content />
                    }
                }
                .await,
                unlisted,
            });
        }
        articles.sort_by(|a, b| b.datetime.cmp(&a.datetime));

        Self { articles }
    }

    pub fn list(&'a self) -> Vec<&'a Article> {
        self.articles.iter().filter(|a| !a.unlisted).collect()
    }

    pub fn get_article_by_slug(&self, slug: &str) -> Option<&Article> {
        self.articles.iter().find(|p| p.slug == slug)
    }
}
