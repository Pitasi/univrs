use maud::{html, Markup};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(PartialEq, PartialOrd)]
enum ImgFormat {
    Svg(String),
    Avif(String),
    Webp(String),
    Png(String),
    Jpeg(String),
}

impl ImgFormat {
    fn path(&self) -> &str {
        match self {
            ImgFormat::Svg(p) => p,
            ImgFormat::Avif(p) => p,
            ImgFormat::Webp(p) => p,
            ImgFormat::Png(p) => p,
            ImgFormat::Jpeg(p) => p,
        }
    }

    fn mime_type(&self) -> &'static str {
        match self {
            ImgFormat::Svg(_) => "image/svg+xml",
            ImgFormat::Avif(_) => "image/avif",
            ImgFormat::Webp(_) => "image/webp",
            ImgFormat::Png(_) => "image/png",
            ImgFormat::Jpeg(_) => "image/jpeg",
        }
    }
}

impl From<&PathBuf> for ImgFormat {
    fn from(path: &PathBuf) -> Self {
        let p = path.to_str().unwrap().to_string();
        match path.extension().unwrap().to_str().unwrap() {
            "svg" => ImgFormat::Svg(p),
            "avif" => ImgFormat::Avif(p),
            "webp" => ImgFormat::Webp(p),
            "png" => ImgFormat::Png(p),
            "jpg" | "jpeg" => ImgFormat::Jpeg(p),
            _ => unreachable!(),
        }
    }
}

fn search_available_sources(file_path: &str) -> Vec<ImgFormat> {
    let path = Path::new(file_path);

    let mut sources = vec![];

    let tries = vec![
        path.with_extension("svg"),
        path.with_extension("avif"),
        path.with_extension("webp"),
        path.with_extension("png"),
        path.with_extension("jpg"),
    ];

    for t in tries {
        if fs::metadata(&t).is_ok() {
            sources.push(ImgFormat::from(&t));
        }
    }

    sources
}

pub fn static_img(path: &str, alt: &str, class: &str) -> Markup {
    let sources = search_available_sources(path);
    if sources.is_empty() {
        panic!("couldn't find any image source for {}", path);
    }

    let (fallback, sources) = sources.split_last().unwrap();

    html! {
        picture class="contents" {
            @for source in sources {
                source srcset=(format!("/{}", source.path())) type=(source.mime_type());
            }
            img src=(format!("/{}", fallback.path())) class=(class) alt=(alt) loading="lazy" decoding="async";
        }
    }
}
