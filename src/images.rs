use std::fs;
use std::path::{Path, PathBuf};
use sycamore::prelude::*;

#[derive(Clone, PartialEq, PartialOrd)]
pub enum ImageSrc {
    Svg(String),
    Avif(String),
    Webp(String),
    Png(String),
    Jpeg(String),
}

impl ImageSrc {
    pub fn path(&self) -> &str {
        match self {
            ImageSrc::Svg(p) => p,
            ImageSrc::Avif(p) => p,
            ImageSrc::Webp(p) => p,
            ImageSrc::Png(p) => p,
            ImageSrc::Jpeg(p) => p,
        }
    }

    pub fn mime_type(&self) -> &'static str {
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

pub fn search_available_sources(file_path: &str) -> Vec<ImageSrc> {
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

#[derive(Props)]
pub struct StaticImgProps {
    path: String,
    alt: String,
    class: String,
}

#[component]
pub fn StaticImg<G: Html>(cx: Scope, props: StaticImgProps) -> View<G> {
    let mut sources = search_available_sources(&props.path);
    if sources.is_empty() {
        panic!("couldn't find any image source for {}", props.path);
    }

    let fallback = sources.pop().unwrap();

    let sources_elements = View::new_fragment(
        sources
            .into_iter()
            .map(|s| {
                let path = format!("/{}", s.path());
                let mime_type = s.mime_type();
                view! { cx,
                    source(
                        srcset=(path),
                        type=(mime_type)
                    ) {}
                }
            })
            .collect::<Vec<_>>(),
    );

    view! {
        cx,
        picture(class="contents") {
            (sources_elements)

            img(
                src=format!("/{}", fallback.path()),
                class=(props.class),
                alt=(props.alt),
                loading="lazy",
                decoding="async"
            ) {}
        }
    }
}

#[derive(Props)]
pub struct RemoteImgProps {
    srcset: Srcset,
    alt: String,
    class: String,
}

#[component]
pub fn RemoteImg<G: Html>(cx: Scope, props: RemoteImgProps) -> View<G> {
    let mut sources: Vec<ImageSrc> = props.srcset.into();
    if sources.is_empty() {
        panic!("there must be at least one source");
    }

    let fallback = sources.pop().unwrap();

    let sources_elements = View::new_fragment(
        sources
            .into_iter()
            .map(|s| {
                let mime_type = s.mime_type();
                view! { cx,
                    source( srcset=s.path(), type=mime_type) {}
                }
            })
            .collect::<Vec<_>>(),
    );

    view! { cx,
        picture(class="contents") {
            (sources_elements)
            img(
                src=fallback.path(),
                class=props.class,
                alt=props.alt,
                loading="lazy",
                decoding="async"
            )
        }
    }
}
