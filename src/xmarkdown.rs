use comrak::{markdown_to_html, ComrakOptions};
use lol_html::{
    element,
    html_content::{ContentType, Element},
    Settings,
};
use maud::{html, Markup, Render};

use crate::rsc::apply;

#[derive(Debug, Clone)]
pub struct Markdown(pub String);

impl Render for Markdown {
    fn render(&self) -> maud::Markup {
        let options = ComrakOptions {
            render: comrak::ComrakRenderOptions {
                unsafe_: true,
                ..comrak::ComrakRenderOptions::default()
            },
            ..comrak::ComrakOptions::default()
        };
        let html = markdown_to_html(&self.0, &options);
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

pub struct EnhancedMd(pub Markdown);

impl From<String> for EnhancedMd {
    fn from(s: String) -> Self {
        Self(Markdown(s))
    }
}

impl Render for EnhancedMd {
    fn render(&self) -> Markup {
        apply(
            Settings {
                element_content_handlers: vec![element!("alert", |el| {
                    let msg = el.get_attribute("msg").expect("msg attribute is required");
                    el.replace_comp(html! {
                        div style="background: red; padding: 10px;" { (msg) }
                    });
                    Ok(())
                })],
                ..Settings::default()
            },
            html! {
                (&self.0)
            },
        )
    }
}
