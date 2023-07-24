use std::fs::read_to_string;

use comrak::plugins::syntect::SyntectAdapter;
use comrak::{markdown_to_html_with_plugins, ComrakOptions, ComrakPlugins};

use crate::xmarkdown::Markdown;

pub struct MarkdownFile {
    pub name: String,
    pub content: Markdown,
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
                content: Markdown(content),
            })
        })
        .collect::<Vec<_>>()
}

pub fn parse(input: &str) -> (frontmatter::Yaml, String) {
    let (frontmatter, input) = parse_frontmatter(input);

    let adapter = SyntectAdapter::new("InspiredGitHub");

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
    let mut plugins = ComrakPlugins::default();
    plugins.render.codefence_syntax_highlighter = Some(&adapter);

    let html = markdown_to_html_with_plugins(input, &options, &plugins);
    (frontmatter, html)
}

fn parse_frontmatter(input: &str) -> (frontmatter::Yaml, &str) {
    let (fm, content) = frontmatter::parse_and_find_content(input).unwrap();
    (fm.unwrap(), content)
}
