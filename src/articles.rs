use crate::markdown::load_dir;

pub struct Article {
    pub title: String,
    pub datetime: String,
    pub slug: String,
    pub content: String,
}

pub fn list_articles() -> Vec<Article> {
    load_dir("./articles")
        .into_iter()
        .map(|md| {
            let title = md.frontmatter["title"].as_str().unwrap().to_string();
            let datetime = md.frontmatter["datetime"].as_str().unwrap().to_string();
            Article {
                title,
                datetime,
                slug: md.name.clone(),
                content: md.content,
            }
        })
        .collect::<Vec<_>>()
}

pub fn get_article_by_slug(slug: String) -> Option<Article> {
    list_articles().into_iter().find(|p| p.slug == slug)
}
