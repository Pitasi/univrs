use std::fs::read_to_string;

use comrak::{markdown_to_html, ComrakOptions};

use crate::rsc::render;

pub struct MarkdownFile {
    pub name: String,
    pub content: String,
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
            let (frontmatter, content) = parse(&input);
            Some(MarkdownFile {
                name,
                frontmatter,
                content,
            })
        })
        .collect::<Vec<_>>()
}

pub fn parse(input: &str) -> (frontmatter::Yaml, String) {
    let (frontmatter, input) = parse_frontmatter(input);

    let options = ComrakOptions {
        parse: comrak::ComrakParseOptions {
            ..comrak::ComrakParseOptions::default()
        },

        extension: comrak::ComrakExtensionOptions {
            autolink: true,
            table: true,
            description_lists: true,
            superscript: true,
            strikethrough: true,
            footnotes: true,
            ..comrak::ComrakExtensionOptions::default()
        },

        render: comrak::ComrakRenderOptions {
            unsafe_: true,
            ..comrak::ComrakRenderOptions::default()
        },
    };

    let html = markdown_to_html(input, &options);
    (frontmatter, render(&html))
}

fn parse_frontmatter(input: &str) -> (frontmatter::Yaml, &str) {
    let (fm, content) = frontmatter::parse_and_find_content(input).unwrap();
    (fm.unwrap(), content)
}
