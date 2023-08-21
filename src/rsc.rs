use crate::components::md::Dialog;
use crate::images::{self, RemoteImg};
use lol_html::html_content::ContentType;
use lol_html::{element, HtmlRewriter, Settings};
use sycamore::prelude::*;

pub fn apply<'s, 'h>(settings: Settings<'s, 'h>, html: &str) -> String {
    let mut output = vec![];
    let mut rewriter = HtmlRewriter::new(settings, |c: &[u8]| output.extend_from_slice(c));
    rewriter.write(html.as_bytes()).unwrap();
    rewriter.end().unwrap();
    String::from_utf8(output).unwrap()
}

pub fn render(html: &str) -> String {
    apply(
        Settings {
            element_content_handlers: vec![
                element!("image", |el| {
                    let avif = el.get_attribute("avif");
                    let webp = el.get_attribute("webp");
                    let png = el.get_attribute("png");
                    let jpeg = el.get_attribute("jpeg");
                    let svg = el.get_attribute("svg");
                    let srcset = images::Srcset {
                        avif,
                        webp,
                        png,
                        jpeg,
                        svg,
                    };
                    let alt = el.get_attribute("alt").unwrap_or(String::new());
                    let class = el.get_attribute("class").unwrap_or(String::new());

                    let comp = sycamore::render_to_string(|cx| {
                        view! {cx, RemoteImg(srcset=srcset, alt=alt, class=class) }
                    });
                    el.replace(&comp, ContentType::Html);

                    Ok(())
                }),
                element!("dialog", |el| {
                    let character = el
                        .get_attribute("character")
                        .expect("missing required attribute 'character' for 'dialog'");

                    let pos = el
                        .get_attribute("pos")
                        .expect("missing required attribute 'pos' for 'dialog'");

                    let msg = el
                        .get_attribute("msg")
                        .expect("missing required attribute 'msg' for 'dialog'");

                    let parse = el.get_attribute("parse").unwrap_or("html".to_string());

                    let comp = sycamore::render_to_string(|cx| {
                        view! {cx, Dialog(character=character, pos=pos, parse=parse, msg=msg) }
                    });
                    let res = render(&comp);

                    el.replace(&res, ContentType::Html);
                    Ok(())
                }),
            ],
            ..Settings::default()
        },
        html,
    )
}
