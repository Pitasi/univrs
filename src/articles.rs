use chrono::{DateTime, FixedOffset};

use crate::{markdown::load_dir, xmarkdown::Markdown};

#[derive(Clone, Debug)]
pub struct Article {
    pub title: String,
    pub datetime: DateTime<FixedOffset>,
    pub slug: String,
    pub content: Markdown,
    pub unlisted: bool,
}

#[derive(Clone, Debug)]
pub struct ArticlesRepo {
    pub articles: Vec<Article>,
}

impl<'a> ArticlesRepo {
    pub fn new() -> Self {
        let mut articles = load_dir("./articles")
            .into_iter()
            .map(|md| {
                let title = md.frontmatter["title"].as_str().unwrap().to_string();
                let datetime_str = md.frontmatter["datetime"].as_str().unwrap();
                let datetime = DateTime::parse_from_rfc3339(datetime_str).unwrap();
                let unlisted = md.frontmatter["unlisted"].as_bool().unwrap_or(false);
                Article {
                    title,
                    datetime,
                    slug: md.name.clone(),
                    content: md.content,
                    unlisted,
                }
            })
            .collect::<Vec<_>>();
        articles.sort_by(|a, b| b.datetime.cmp(&a.datetime));

        Self { articles }
    }

    pub fn list(&'a self) -> Vec<&'a Article> {
        self.articles.iter().filter(|a| !a.unlisted).collect()
    }

    pub fn get_article_by_slug(&self, slug: String) -> Option<&Article> {
        self.articles.iter().find(|p| p.slug == slug)
    }
}

impl Default for ArticlesRepo {
    fn default() -> Self {
        Self::new()
    }
}
