use maud::{html, Markup};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(PartialEq, PartialOrd)]
pub enum ImageSrc {
    Svg(String),
    Avif(String),
    Webp(String),
    Png(String),
    Jpeg(String),
}

impl ImageSrc {
    fn path(&self) -> &str {
        match self {
            ImageSrc::Svg(p) => p,
            ImageSrc::Avif(p) => p,
            ImageSrc::Webp(p) => p,
            ImageSrc::Png(p) => p,
            ImageSrc::Jpeg(p) => p,
        }
    }

    fn mime_type(&self) -> &'static str {
        match self {
            ImageSrc::Svg(_) => "image/svg+xml",
            ImageSrc::Avif(_) => "image/avif",
            ImageSrc::Webp(_) => "image/webp",
            ImageSrc::Png(_) => "image/png",
            ImageSrc::Jpeg(_) => "image/jpeg",
        }
    }
}

impl From<&PathBuf> for ImageSrc {
    fn from(path: &PathBuf) -> Self {
        let p = path.to_str().unwrap().to_string();
        match path.extension().unwrap().to_str().unwrap() {
            "svg" => ImageSrc::Svg(p),
            "avif" => ImageSrc::Avif(p),
            "webp" => ImageSrc::Webp(p),
            "png" => ImageSrc::Png(p),
            "jpg" | "jpeg" => ImageSrc::Jpeg(p),
            _ => unreachable!(),
        }
    }
}

fn search_available_sources(file_path: &str) -> Vec<ImageSrc> {
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
            sources.push(ImageSrc::from(&t));
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

#[derive(Default)]
pub struct Srcset {
    pub svg: Option<String>,
    pub avif: Option<String>,
    pub webp: Option<String>,
    pub png: Option<String>,
    pub jpeg: Option<String>,
}

impl Into<Vec<ImageSrc>> for Srcset {
    fn into(self) -> Vec<ImageSrc> {
        let mut sources = vec![];

        if let Some(p) = self.svg {
            sources.push(ImageSrc::Svg(p));
        }
        if let Some(p) = self.avif {
            sources.push(ImageSrc::Avif(p));
        }
        if let Some(p) = self.webp {
            sources.push(ImageSrc::Webp(p));
        }
        if let Some(p) = self.png {
            sources.push(ImageSrc::Png(p));
        }
        if let Some(p) = self.jpeg {
            sources.push(ImageSrc::Jpeg(p));
        }

        sources
    }
}

pub fn remote_img(srcset: Srcset, alt: &str, class: &str) -> Markup {
    let sources: Vec<ImageSrc> = srcset.into();
    if sources.is_empty() {
        panic!("there must be at least one source");
    }
    let (fallback, sources) = sources.split_last().unwrap();
    html! {
        picture class="contents" {
            @for source in sources {
                source srcset=(source.path()) type=(source.mime_type());
            }
            img src=(fallback.path()) class=(class) alt=(alt) loading="lazy" decoding="async";
        }
    }
}
