use crate::images;
use lol_html::html_content::ContentType;
use lol_html::{element, HtmlRewriter, Settings};
use maud::{Markup, PreEscaped};

pub fn apply<'s, 'h>(settings: Settings<'s, 'h>, html: Markup) -> Markup {
    let mut output = vec![];
    let mut rewriter = HtmlRewriter::new(settings, |c: &[u8]| output.extend_from_slice(c));
    rewriter.write(html.0.as_bytes()).unwrap();
    rewriter.end().unwrap();
    PreEscaped(String::from_utf8(output).unwrap())
}

#[tracing::instrument(level = "info")]
pub fn render(html: Markup) -> Markup {
    apply(
        Settings {
            element_content_handlers: vec![element!("image", |el| {
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
                let alt = el.get_attribute("alt").unwrap_or("".to_string());
                let class = el.get_attribute("class").unwrap_or("".to_string());
                let res = render(PreEscaped(
                    images::remote_img(srcset, &alt, &class).into_string(),
                ));
                el.replace(&res.0, ContentType::Html);
                Ok(())
            })],
            ..Settings::default()
        },
        html,
    )
}
