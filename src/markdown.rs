use crate::rsc;
use comrak::{
    markdown_to_html_with_plugins, plugins::syntect::SyntectAdapter, ComrakOptions, ComrakPlugins,
};
use std::fs::read_to_string;

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
            let (frontmatter, content) = parse_frontmatter(&input);
            Some(MarkdownFile {
                name,
                frontmatter,
                content: content.to_string(),
            })
        })
        .collect::<Vec<_>>()
}

fn parse_frontmatter(input: &str) -> (frontmatter::Yaml, &str) {
    let (fm, content) = frontmatter::parse_and_find_content(input).unwrap();
    (fm.unwrap(), content)
}

pub fn parse(s: &str) -> String {
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

    let adapter = SyntectAdapter::new("InspiredGitHub");
    plugins.render.codefence_syntax_highlighter = Some(&adapter);

    let html = markdown_to_html_with_plugins(s, &options, &plugins);
    html
}

pub fn parse_with_custom_components(s: &str) -> String {
    let html = parse(s);
    rsc::render(&html)
}
