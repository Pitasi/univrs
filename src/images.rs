use rscx::*;
use std::fmt::Write;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum ImageSrc {
    Svg(String),
    Avif(String),
    Webp(String),
    Png(String),
    Jpeg(String),
}

impl ImageSrc {
    pub fn path(&self) -> String {
        let p = match self {
            ImageSrc::Svg(p) => p,
            ImageSrc::Avif(p) => p,
            ImageSrc::Webp(p) => p,
            ImageSrc::Png(p) => p,
            ImageSrc::Jpeg(p) => p,
        };
        if p.starts_with("http") || p.starts_with("/") {
            p.into()
        } else {
            format!("/{}", p)
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

impl From<String> for ImageSrc {
    fn from(path: String) -> Self {
        match Path::new(&path).extension().unwrap().to_str().unwrap() {
            "svg" => ImageSrc::Svg(path),
            "avif" => ImageSrc::Avif(path),
            "webp" => ImageSrc::Webp(path),
            "png" => ImageSrc::Png(path),
            "jpg" | "jpeg" => ImageSrc::Jpeg(path),
            _ => unreachable!(),
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

#[props]
pub struct StaticImgProps {
    path: String,
    alt: String,
    class: String,
}

#[component]
pub fn StaticImg(props: StaticImgProps) -> String {
    let sources = search_available_sources(&props.path);
    if sources.is_empty() {
        panic!("couldn't find any image source for {}", props.path);
    }

    html! {
        <Image sources=sources alt=props.alt class=props.class />
    }
}

#[props]
pub struct RemoteImgProps {
    srcset: Srcset,
    alt: String,
    class: String,
}

#[component]
pub async fn RemoteImg(props: RemoteImgProps) -> String {
    let sources: Vec<ImageSrc> = props.srcset.into();
    html! {
        <Image sources=sources alt=props.alt class=props.class />
    }
}

#[props]
pub struct ImageProps {
    sources: Vec<ImageSrc>,
    alt: String,
    class: String,
}

#[component]
pub async fn Image(mut props: ImageProps) -> String {
    if props.sources.is_empty() {
        panic!("there must be at least one source");
    }

    let fallback = props.sources.pop().unwrap();

    let sources_elements = props
        .sources
        .into_iter()
        .map(|s| {
            let mime_type = s.mime_type();
            html! {
                <source srcset=s.path() type=mime_type />
            }
        })
        .collect_fragment();

    let mut class = String::new();
    if props.class.contains("h-full") {
        class.write_str("h-full ").unwrap();
    }
    if props.class.contains("w-full") {
        class.write_str("w-full ").unwrap();
    }

    html! {
        <picture class=class>
            {sources_elements}
            <img src=fallback.path()
                class=props.class
                alt=props.alt
                loading="lazy"
                decoding="async"
            />
        </picture>
    }
}
