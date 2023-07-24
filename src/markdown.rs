use std::fs::read_to_string;

use crate::xmarkdown::{EnhancedMd, Markdown};

pub struct MarkdownFile {
    pub name: String,
    pub content: EnhancedMd,
    pub frontmatter: frontmatter::Yaml,
}

pub fn load_dir(path: &str) -> Vec<MarkdownFile> {
    std::fs::read_dir(path)
        .unwrap()
        .filter_map(|entry| {
            let entry = entry.unwrap();
            let path = entry.path();
            match path.extension() {
                Some(ext) if ext == "md" => {}
                _ => return None,
            }

            let name = path.file_stem().unwrap().to_str().unwrap().to_string();
            let input = read_to_string(path).unwrap();
            let (frontmatter, content) = parse_frontmatter(&input);
            Some(MarkdownFile {
                name,
                frontmatter,
                content: EnhancedMd(Markdown(content.into())),
            })
        })
        .collect::<Vec<_>>()
}

fn parse_frontmatter(input: &str) -> (frontmatter::Yaml, &str) {
    let (fm, content) = frontmatter::parse_and_find_content(input).unwrap();
    (fm.unwrap(), content)
}
