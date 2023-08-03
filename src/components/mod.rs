use std::{fs::File, io::BufReader};

use axum::http;
use data_encoding::HEXLOWER;
use maud::{html, Markup, PreEscaped, Render};

use crate::{hash, icons, images, pages::auth::User};

#[derive(Default, Debug)]
pub struct Meta<'a> {
    pub title: Option<&'a str>,
    pub description: Option<&'a str>,
}

impl<'a> Render for Meta<'a> {
    fn render(&self) -> Markup {
        let title = match self.title {
            Some(title) => format!("{} - Antonio Pitasi", title),
            None => "Antonio Pitasi".into(),
        };
        let description = match self.description {
            Some(description) => description,
            None => "Antonio's personal website, a backend software engineer passionate about distributed systems and clean, maintainable software. Currently working at Qredo, aiming to decentralize the private keys for cryptocurrencies. Founder of the local community, pisa.dev.",
        };

        html! {
            title { (title) }
            meta charset="utf-8";
            meta name="viewport" content="width=device-width, initial-scale=1";
            meta name="description" content=(description);
        }
    }
}

struct StaticFile<'a>(&'a str);

impl<'a> Render for StaticFile<'a> {
    fn render(&self) -> Markup {
        let path = format!("static/{}", self.0);
        let input = File::open(&path).expect(format!("failed to open file: {}", &path).as_str());
        let reader = BufReader::new(input);
        let digest = hash::sha256_digest(reader).expect("failed to hash css file");
        let file_url = format!("/{}?{}", path, &HEXLOWER.encode(digest.as_ref())[..5]);
        PreEscaped(file_url)
    }
}

struct CssFile<'a>(StaticFile<'a>);

impl<'a> From<&'a str> for CssFile<'a> {
    fn from(s: &'a str) -> Self {
        Self(StaticFile(s))
    }
}

impl<'a> Render for CssFile<'a> {
    fn render(&self) -> Markup {
        html! {
            link rel="stylesheet" type="text/css" href=(self.0);
        }
    }
}

fn favicon() -> Markup {
    html! {
      link rel="apple-touch-icon" sizes="180x180" href="/static/apple-touch-icon.png";
      link rel="icon" type="image/png" sizes="32x32" href="/static/favicon-32x32.png";
      link rel="icon" type="image/png" sizes="16x16" href="/static/favicon-16x16.png";
      link rel="manifest" href="/static/site.webmanifest";
      link rel="mask-icon" href="/static/safari-pinned-tab.svg" color="#9e9815";
      meta name="msapplication-TileColor" content="#da532c";
      meta name="theme-color" content="#ffffff";
    }
}

// end maud helpers

#[tracing::instrument(level = "info")]
pub fn root(uri: &http::Uri, meta: Meta, slot: Markup, user: Option<User>) -> Markup {
    let res = html! {
        (maud::DOCTYPE)
        html lang="en" ."bg-floralwhite" {
            head {
                (meta)
                (favicon())
                (CssFile::from("style.dist.css"))
                (CssFile::from("tailwind.css"))
                script defer data-domain="anto.pt" src="https://plausible.anto.pt/js/plausible.js" {}
                link rel="preload" href="/static/Inter-VariableFont_slnt,wght.ttf" crossorigin="anonymous" as="font" type="font/ttf";
                link rel="preload" href="/static/DarkerGrotesque-VariableFont_wght.ttf" crossorigin="anonymous" as="font" type="font/ttf";
            }
            body class="flex min-h-screen" hx-ext="loading-states" {
              script src="/static/anime.min.js" {}
              script src="/static/htmx.min.js" integrity="sha384-L6OqL9pRWyyFU3+/bjdSri+iIphTN/bvYyM37tICVyOJkWZLpP2vGn6VUEXgzg6h" crossorigin="anonymous" {}
              script src="/static/htmx-loading-states.js" {}
              .flex ."flex-1" .flex-row {
                (mobile_navbar(uri, user.clone()))
                (root_sidebar(uri, user))
                (slot)
              }
            }
        }
    };

    res
}

pub fn mobile_navbar(uri: &http::Uri, user: Option<User>) -> Markup {
    html! {
        nav class="fixed bottom-0 left-0 z-10 flex h-12 w-full flex-row items-center justify-start border-t-2 border-black bg-acid px-2 lg:hidden" {
            button class="inline-flex items-center justify-center rounded-md text-sm font-medium transition-colors focus:outline-none focus:ring-2 focus:ring-offset0 disabled:opacity-50 disabled:pointer-events-none data-[state=open]:bg-slate-100 h-10 py-2 px-4"
            onClick="document.getElementById('xxx').dataset.state = 'open';" {
                (icons::burger())
            }
        }

        div id="xxx" data-state="closed" class="data-[state=closed]:hidden fixed inset-0 z-50 flex items-start justify-center sm:items-center" {
            div class="fixed inset-0 z-50 bg-black/50 backdrop-blur-sm transition-all duration-100 data-[state=closed]:animate-out data-[state=open]:fade-in" data-aria-hidden="true" aria-hidden="true" style="pointer-events: auto;" { }

            div data-state="open" class="fixed z-50 grid w-full gap-4 rounded-b-lg border-black bg-white shadow-neu-3 animate-in data-[state=open]:fade-in-90 data-[state=open]:slide-in-from-bottom-10 sm:rounded-lg sm:zoom-in-90 data-[state=open]:sm:slide-in-from-bottom-0 bottom-0 top-auto border-0 p-0 sm:max-w-full lg:hidden" {
                div class="sticky bottom-0 top-0 max-h-screen overflow-auto p-4 lg:border-r-2 w-full space-y-20 border-t-2 border-black bg-lightviolet bg-pattern-hideout pb-10" {
                    div class="space-y-10" {
                        (sidebar_header(Some("static/bulb.webp"), "Antonio Pitasi"))
                        (root_sidebar_nav(uri))
                    }
                    (login_widget(user))
                }

                button type="button" class="absolute top-4 right-4 rounded-sm opacity-70 transition-opacity hover:opacity-100 focus:outline-none focus:ring-2 focus:ring-slate-400 focus:ring-offset-2 disabled:pointer-events-none data-[state=open]:bg-slate-100 dark:focus:ring-slate-400 dark:focus:ring-offset-slate-900 dark:data-[state=open]:bg-slate-800"
                onClick="document.getElementById('xxx').dataset.state = 'closed';" {
                    ( icons::small_x() )
                    span class="sr-only" { "Close" }
                }
            }
        }
    }
}

pub fn root_sidebar_nav(uri: &http::Uri) -> Markup {
    let nav = vec![
        ("Home", "/", Some(icons::home())),
        ("Articles", "/articles", Some(icons::pen())),
    ];

    html! {
        (sidebar_nav(html! {
            @for (name, href, icon) in nav.iter() {
                (sidebar_nav_item(href, icon, html! {
                    (name)
                }, is_active(uri.path(), href)))
            }
        }))
    }
}

pub fn root_sidebar(uri: &http::Uri, user: Option<User>) -> Markup {
    html! {
        div class="
            hidden w-48 shrink-0 bg-lightviolet bg-pattern-hideout
            lg:flex lg:flex-col lg:justify-between sticky bottom-0 top-0
            max-h-screen space-y-8 overflow-auto border-black p-4
            lg:border-r-2
        " {
            div class="space-y-8" {
                (sidebar_header(Some("static/bulb.webp"), "Antonio Pitasi"))
                (root_sidebar_nav(&uri))
            }
            (login_widget(user))
        }
    }
}

pub fn is_active(path: &str, href: &str) -> bool {
    match href {
        "/" => path == "/",
        _ => path.starts_with(href),
    }
}

pub fn secondary_sidebar(slot: Markup) -> Markup {
    html! {
        div class="
sticky bottom-0 top-0 max-h-screen w-full space-y-8 overflow-auto border-black p-4 lg:border-r-2 min-h-full shrink-0 bg-pattern-hideout pb-24 lg:block lg:min-h-0 lg:pb-0
        " {
            div class="space-y-8" {
                (sidebar_header(None, "Articles"))
                (sidebar_nav(slot))
            }
        }
    }
}

pub fn sidebar_header(img_src: Option<&str>, title: &str) -> Markup {
    html! {
        header class="flex h-10 w-full flex-row items-center gap-1" {
            @if let Some(src) = img_src {
                div class="flex h-10 w-10 shrink-0 items-center justify-center rounded-xl" {
                    (images::static_img(src, title, "w-full h-full object-contain"))
                }
            } @else {
                div class="h-10 w-4" { }
            }
            div class="flex flex-col font-neu text-xl font-bold leading-none tracking-tight text-black" {
                span { (title) }
            }
        }
    }
}

pub fn sidebar_nav(slot: Markup) -> Markup {
    html! {
        ul ."space-y-4" {
            (slot)
        }
    }
}

pub fn sidebar_nav_item(href: &str, icon: &Option<Markup>, slot: Markup, active: bool) -> Markup {
    html! {
        li {
            a href=(href) class="
                rounded-md text-sm font-medium transition-colors focus:outline-none
                focus:ring-offset0 disabled:opacity-50 disabled:pointer-events-none
                data-[state=open]:bg-slate-100 py-2 px-4 flex h-auto w-full flex-row items-center
                justify-start ring-inset ring-lightviolet ring-offset-transparent focus:ring-1
                data-active:translate-x-0 data-active:translate-y-0 data-active:border-black
                data-active:bg-yellow data-active:shadow-none -translate-x-0.5 -translate-y-0.5
                border-2 border-black bg-white shadow-neu-2 hover:translate-x-0 hover:translate-y-0
                hover:shadow-none
            " data-active=(active) {
                @if let Some(icon) = icon {
                    span class="mr-2 h-4 w-4" { (icon) }
                }
                (slot)
            }
        }
    }
}

pub fn login_widget(user: Option<User>) -> Markup {
    html! {
        div class="w-full" {
            @match user {
                Some(user) => {
                    div class="flex flex-row justify-between" {
                        div class="w-10" {
                            span class="relative flex shrink-0 overflow-hidden rounded-md row-span-2 aspect-square h-auto w-full border-2 border-black" {
                                        img class="aspect-square h-full w-full" src=(user.picture.unwrap_or("/static/bulb.webp".to_string()));
                            }
                        }
                        a class="inline-flex items-center justify-center rounded-md text-sm font-medium transition-colors focus:outline-none focus:ring-2 focus:ring-offset0 disabled:opacity-50 disabled:pointer-events-none data-[state=open]:bg-slate-100 bg-floralwhite text-black hover:bg-slate-200 h-10 w-10 border-2 border-black"
                            href="/auth/logout" {
                            (icons::logout())
                        }
                    }
                }

                None => {
                    a class="inline-flex items-center justify-center rounded-md text-sm font-medium transition-colors focus:outline-none focus:ring-2 focus:ring-offset0 disabled:opacity-50 disabled:pointer-events-none data-[state=open]:bg-slate-100 bg-floralwhite text-black hover:bg-slate-200 h-10 py-2 px-4 w-full border-2 border-black"
                        href="/auth/login" { "Sign in" }
                }
            }
        }
    }
}
