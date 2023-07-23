use lol_html::html_content::ContentType;
use lol_html::{element, HtmlRewriter, Settings};
use crate::images;

macro_rules! component {
    ($tag:ident) => {
        component_def!(
            $tag,
            include_str!(concat!("components/", stringify!($tag), ".html"))
        )
    };
    ($tag:ident, $p1:ident) => {
        component_def!(
            $tag,
            include_str!(concat!("components/", stringify!($tag), ".html")),
            $p1
        )
    };
}

macro_rules! component_def {
    ($tag:ident, $def:expr) => {
        element!(stringify!($tag), |el| {
            let res = render($def);
            el.replace(&res, ContentType::Html);
            Ok(())
        })
    };
    ($tag:ident, $def:expr, $p1:ident) => {
        element!(stringify!($tag), |el| {
            let $p1 = el.get_attribute(stringify!($p1)).expect(&format!(
                "missing required attribute for {}: {}",
                stringify!($tag),
                stringify!($p1)
            ));
            let res = render(&format!($def, $p1 = $p1));
            el.replace(&res, ContentType::Html);
            Ok(())
        })
    };
}

#[tracing::instrument(level = "info")]
pub fn render(html: &str) -> String {
    let mut output = vec![];
    let mut rewriter = HtmlRewriter::new(
        Settings {
            element_content_handlers: vec![
                element!("image", |el| {
                    let avif = el.get_attribute("avif");
                    let webp = el.get_attribute("webp");
                    let png = el.get_attribute("png");
                    let jpeg = el.get_attribute("jpeg");
                    let svg = el.get_attribute("svg");
                    let srcset = images::Srcset{
                        avif,
                        webp,
                        png,
                        jpeg,
                        svg,
                    };
                    let alt = el.get_attribute("alt").unwrap_or("".to_string());
                    let class = el.get_attribute("class").unwrap_or("".to_string());
                    let res = render(images::remote_img(srcset, &alt, &class).into_string().as_str());
                    el.replace(&res, ContentType::Html);
                    Ok(())
                }),
            ],
            ..Settings::default()
        },
        |c: &[u8]| output.extend_from_slice(c),
    );
    rewriter.write(html.as_bytes()).unwrap();
    rewriter.end().unwrap();
    String::from_utf8(output).unwrap()
}

