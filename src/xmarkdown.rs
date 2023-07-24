use comrak::{
    markdown_to_html_with_plugins, plugins::syntect::SyntectAdapter, ComrakOptions, ComrakPlugins,
};
use lol_html::html_content::{ContentType, Element};
use maud::{html, Markup, Render};

use crate::rsc;

#[derive(Debug, Clone)]
pub struct Markdown(pub String);

impl Render for Markdown {
    fn render(&self) -> maud::Markup {
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

        let html = markdown_to_html_with_plugins(&self.0, &options, &plugins);
        maud::PreEscaped(html)
    }
}

pub trait ComponentReplacer {
    fn replace_comp(&mut self, comp: Markup);
}

impl ComponentReplacer for Element<'_, '_> {
    fn replace_comp(&mut self, comp: Markup) {
        self.replace(&comp.0, ContentType::Html);
    }
}

#[derive(Debug, Clone)]
pub struct EnhancedMd(pub Markdown);

impl From<String> for EnhancedMd {
    fn from(s: String) -> Self {
        Self(Markdown(s))
    }
}

impl Render for EnhancedMd {
    fn render(&self) -> Markup {
        rsc::render(html! { (self.0) })
    }
}
