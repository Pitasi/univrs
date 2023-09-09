use std::fs::read_to_string;

use rscx::{component, html, props};
use rscx_mdx::mdx::{Mdx, MdxComponentProps, MdxProps};

use crate::{
    components::md::{Dialog, DialogProps},
    images::{RemoteImg, RemoteImgProps, Srcset},
};

pub struct MarkdownFile {
    pub name: String,
    pub content: String,
    pub frontmatter: frontmatter::Yaml,
}

pub fn load_dir(path: &str) -> Vec<MarkdownFile> {
    std::fs::read_dir(path)
        .unwrap()
        .filter_map(|entry| {
            let entry = entry.unwrap();
            let path = entry.path();
            match path.extension() {
                Some(ext) if ext == "md" => {}
                _ => return None,
            }

            let name = path.file_stem().unwrap().to_str().unwrap().to_string();
            let input = read_to_string(path).unwrap();
            let (frontmatter, content) = parse_frontmatter(&input);
            Some(MarkdownFile {
                name,
                frontmatter,
                content: content.to_string(),
            })
        })
        .collect::<Vec<_>>()
}

fn parse_frontmatter(input: &str) -> (frontmatter::Yaml, &str) {
    let (fm, content) = frontmatter::parse_and_find_content(input).unwrap();
    (fm.unwrap(), content)
}

async fn handler(name: String, props: MdxComponentProps) -> String {
    match name.as_str() {
        "Dialog" => {
            let character = props.attributes.get("character").unwrap().clone().unwrap();
            let pos = props.attributes.get("pos").unwrap().clone().unwrap();
            html! {
                <Dialog character=character pos=pos>
                    {props.children}
                </Dialog>
            }
        }
        "RemoteImage" => {
            let avif = props.attributes.get("avif").map(|v| v.clone().unwrap());
            let webp = props.attributes.get("webp").map(|v| v.clone().unwrap());
            let png = props.attributes.get("png").map(|v| v.clone().unwrap());
            let jpeg = props.attributes.get("jpeg").map(|v| v.clone().unwrap());
            let svg = props.attributes.get("svg").map(|v| v.clone().unwrap());
            let srcset = Srcset {
                avif,
                webp,
                png,
                jpeg,
                svg,
            };

            let alt = props
                .attributes
                .get("alt")
                .map_or(String::new(), |v| v.clone().unwrap());
            let class = props
                .attributes
                .get("alt")
                .map_or(String::new(), |v| v.clone().unwrap());

            let res = html! {
                <RemoteImg srcset=srcset alt=alt class=class />
            };

            res
        }
        _ => {
            eprintln!("unknown component: {}", name);
            String::new()
        }
    }
}

#[props]
pub struct MarkdownProps {
    source: String,
}

#[component]
pub async fn Markdown(props: MarkdownProps) -> String {
    html! {
        <Mdx handler=handler source=props.source />
    }
}
