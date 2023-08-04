use std::collections::HashSet;

use lightningcss::{
    stylesheet::{MinifyOptions, ParserOptions, PrinterOptions, StyleSheet},
    targets::{Browsers, Targets},
};
use lol_html::{element, html_content::ContentType, text, HtmlRewriter, Settings};
use maud::{html, Markup, PreEscaped, Render};
use stylist::Style;

#[macro_export]
macro_rules! style {
    ($l:tt) => {
        stylist::style!($l).expect("couldn't build style")
    };
}

macro_rules! styled_element {
    ($name:ident, $el:ident) => {
        pub struct $name(pub Style, pub Markup);

        impl Render for $name {
            fn render(&self) -> Markup {
                html! {
                    style scoped { (PreEscaped(self.0.get_style_str())) }
                    $el class=(self.0.get_class_name()) {
                        (self.1)
                    }
                }
            }
        }
    };
}

styled_element!(StyledDiv, div);
styled_element!(StyledLi, li);

pub fn apply_styles(component: Markup) -> Markup {
    let (trimmed_component, styles) = gather_scoped_styles(component);
    let processed_styles = minify(styles);
    add_head_style_tag(trimmed_component, processed_styles)
}

fn gather_scoped_styles(component: Markup) -> (Vec<u8>, String) {
    let mut styles = HashSet::new();

    let s = Settings {
        element_content_handlers: vec![
            element!("style[scoped]", |el| {
                el.remove();
                Ok(())
            }),
            text!("style[scoped]", |t| {
                // Save the text contents for the end tag handler.
                styles.insert(t.as_str().to_string());
                Ok(())
            }),
        ],
        ..Settings::default()
    };

    let mut output = vec![];
    let mut rewriter = HtmlRewriter::new(s, |c: &[u8]| output.extend_from_slice(c));
    rewriter.write(component.0.as_bytes()).unwrap();
    rewriter.end().unwrap();

    (
        output,
        styles.into_iter().collect::<Vec<String>>().join(" "),
    )
}

fn minify(s: String) -> String {
    let mut stylesheet = StyleSheet::parse(&s, ParserOptions::default()).unwrap();

    stylesheet
        .minify(MinifyOptions {
            targets: Targets {
                browsers: Some(Browsers {
                    chrome: Some(100),
                    ..Browsers::default()
                }),
                ..Targets::default()
            },
            ..MinifyOptions::default()
        })
        .unwrap();

    let res = stylesheet
        .to_css(PrinterOptions {
            minify: true,
            ..PrinterOptions::default()
        })
        .unwrap();

    res.code
}

fn add_head_style_tag(component: Vec<u8>, styles: String) -> Markup {
    let s = Settings {
        element_content_handlers: vec![element!("style[placeholder]", |el| {
            el.remove_attribute("placeholder");
            el.set_inner_content(&styles, ContentType::Html);
            Ok(())
        })],
        ..Settings::default()
    };

    let mut output = vec![];
    let mut rewriter = HtmlRewriter::new(s, |c: &[u8]| output.extend_from_slice(c));
    rewriter.write(component.as_slice()).unwrap();
    rewriter.end().unwrap();

    html! {
        (PreEscaped(String::from_utf8(output).unwrap()))
    }
}
