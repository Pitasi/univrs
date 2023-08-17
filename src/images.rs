use leptos::{component, view, IntoAttribute, IntoView, Scope};
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

#[component]
pub fn StaticImg(
    cx: Scope,
    #[prop(into)] path: String,
    #[prop(into)] alt: String,
    #[prop(into)] class: String,
) -> impl IntoView {
    let sources = search_available_sources(&path);
    if sources.is_empty() {
        panic!("couldn't find any image source for {}", path);
    }

    let (fallback, sources) = sources.split_last().unwrap();

    view! {
        cx,
        <picture class="contents">
            {sources.into_iter()
                .map(|source| view! {
                    cx,
                    <source srcset=format!("/{}", source.path()) type_=source.mime_type() />
                })
                .collect::<Vec<_>>()}

            <img src=format!("/{}", fallback.path()) class={class} alt=alt loading="lazy" decoding="async" />
        </picture>
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

#[component]
pub fn RemoteImg(cx: Scope, srcset: Srcset, alt: String, class: String) -> impl IntoView {
    let sources: Vec<ImageSrc> = srcset.into();
    if sources.is_empty() {
        panic!("there must be at least one source");
    }
    let (fallback, sources) = sources.split_last().unwrap();
    let fallback_path = fallback.path().to_string();

    view! { cx,
        <picture class="contents">
            {sources.into_iter()
                .map(|source| view! {
                    cx,
                    <source srcset=source.path().to_owned() type_=source.mime_type() />
                })
                .collect::<Vec<_>>()}

            <img src=fallback_path class=class alt=alt loading="lazy" decoding="async" />
        </picture>
    }
}
