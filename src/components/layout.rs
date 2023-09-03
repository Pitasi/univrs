use std::{fs::File, io::BufReader};

use axum::http::{self, Uri};
use data_encoding::HEXLOWER;
use stylist::style;
use sycamore::prelude::*;

use crate::{
    hash,
    icons::{App, Burger, Heart, Home, Logout, Notebook, SmallX},
    images::StaticImg,
    pages::auth::AuthContext,
    sycamore::{Body, Dedup, Head, Html, Metatag, Title},
    xstyledview,
};

#[derive(Props)]
pub struct LayoutProps<'a, G: Html> {
    children: Children<'a, G>,
}

#[component]
pub fn Layout<'a, G: Html>(cx: Scope<'a>, props: LayoutProps<'a, G>) -> View<G> {
    let children = props.children.call(cx);

    view! { cx,
        Html(attr:lang="en", attr:class="bg-floralwhite") {}
        Head {
            Title { "Antonio Pitasi" }
            Favicon {}
            meta(charset="utf-8")
            meta(name="viewport", content="width=device-width, initial-scale=1")
            Metatag(name="description".into(), attr:content="Antonio's personal website, a backend software engineer passionate about clean, maintainable software. Currently working at Qredo, aiming to decentralize the private keys for cryptocurrencies. Founder of the local community, pisa.dev.") {}
            link(rel="preload", href="/static/Inter-VariableFont_slnt,wght.ttf", crossorigin="anonymous", as="font", type_="font/ttf")
            link(rel="preload", href="/static/ClashDisplay-Variable.woff2", crossorigin="anonymous", as="font", type_="font/woff2")
            CssFile(path="style.dist.css".into()) {}
            CssFile(path="tailwind.css".into()) {}
            script(defer=true, data-domain="anto.pt",src="https://plausible.anto.pt/js/plausible.js") {}
        }
        Body(attr:class="flex min-h-screen", attr:hx-ext="loading-states", attr:hx-boost="true") {}

        script(src="/static/anime.min.js") {}
        script(src="/static/htmx.min.js", integrity="sha384-L6OqL9pRWyyFU3+/bjdSri+iIphTN/bvYyM37tICVyOJkWZLpP2vGn6VUEXgzg6h", crossorigin="anonymous", defer=true) {}
        script(src="/static/htmx-loading-states.js", defer=true) {}

        div(class="flex flex-1 flex-row") {
            MobileNavbar {}
            RootSidebar {}
            (children)
        }
    }
}

#[component]
pub fn Favicon<G: Html>(cx: Scope) -> View<G> {
    view! {
        cx,
        link(rel="apple-touch-icon", sizes="180x180", href="/static/apple-touch-icon.png") {}
        link(rel="icon", type_="image/png", sizes="32x32", href="/static/favicon-32x32.png") {}
        link(rel="icon", type_="image/png", sizes="16x16", href="/static/favicon-16x16.png") {}
        link(rel="manifest", href="/static/site.webmanifest") {}
        link(rel="mask-icon", href="/static/safari-pinned-tab.svg") {}
        meta(name="msapplication-TileColor", content="#da532c") {}
        meta(name="theme-color", content="#ffffff") {}
    }
}

#[derive(Props)]
pub struct CssFileProps {
    path: String,
}

#[component]
pub fn CssFile<G: Html>(cx: Scope, props: CssFileProps) -> View<G> {
    let path = format!("static/{}", props.path);
    let input = File::open(&path).expect(format!("failed to open file: {}", &path).as_str());
    let reader = BufReader::new(input);
    let digest = hash::sha256_digest(reader).expect("failed to hash css file");
    let file_url = format!("/{}?{}", path, &HEXLOWER.encode(digest.as_ref())[..5]);

    view! {
        cx,
        link(rel="stylesheet", type="text/css", href=file_url) {}
    }
}

#[component]
pub fn MobileNavbar<G: Html>(cx: Scope) -> View<G> {
    view! {
        cx,
        nav(class="fixed bottom-0 left-0 z-10 flex h-12 w-full flex-row items-center justify-start border-t-2 border-black bg-acid px-2 lg:hidden") {
            button(class="inline-flex items-center justify-center rounded-md text-sm font-medium transition-colors focus:outline-none focus:ring-2 focus:ring-offset-0 disabled:opacity-50 disabled:pointer-events-none data-[state=open]:bg-slate-100 h-10 py-2 px-4",
                onClick="document.getElementById('mobile-dialog').dataset.state = 'open';") {
                Burger {}
            }
        }

        div(id="mobile-dialog", data-state="closed", class="data-[state=closed]:hidden fixed inset-0 z-50 flex items-start justify-center sm:items-center") {
            div(class="fixed inset-0 z-50 bg-black/50 backdrop-blur-sm transition-all duration-100 data-[state=closed]:animate-out data-[state=open]:fade-in") {}
            div(class="fixed z-50 grid w-full gap-4 rounded-b-lg border-black bg-white shadow-neu-3 animate-in data-[state=open]:fade-in-90 data-[state=open]:slide-in-from-bottom-10 sm:rounded-lg sm:zoom-in-90 data-[state=open]:sm:slide-in-from-bottom-0 bottom-0 top-auto border-0 p-0 sm:max-w-full lg:hidden") {
                div(class="sticky bottom-0 top-0 max-h-screen overflow-auto p-4 lg:border-r-2 w-full space-y-20 border-t-2 border-black bg-lightviolet bg-pattern-hideout pb-10") {
                    div(class="space-y-8") {
                        SidebarHeader(img_src="static/bulb.webp".into(), title="Antonio Pitasi".into()) {}
                        RootSidebarNav {}
                    }
                    LoginWidget {}
                }

                button(type_="button", class="absolute top-4 right-4 rounded-sm opacity-70 transition-opacity hover:opacity-100 focus:outline-none focus:ring-2 focus:ring-slate-400 focus:ring-offset-2 disabled:pointer-events-none data-[state=open]:bg-slate-100 dark:focus:ring-slate-400 dark:focus:ring-offset-slate-900 dark:data-[state=open]:bg-slate-800",
                    onClick="document.getElementById('mobile-dialog').dataset.state = 'closed';") {
                    SmallX {}
                    span(class="sr-only") { "Close" }
                }
            }
        }
    }
}

#[component]
pub fn LoginWidget<G: Html>(cx: Scope) -> View<G> {
    let uri = use_context::<http::Uri>(cx);
    let auth = use_context::<AuthContext>(cx);

    if let Some(user) = &auth.current_user {
        let pic = user
            .picture
            .clone()
            .unwrap_or("/static/bulb.webp".to_string());

        view! {
            cx,
            div(class="w-full") {
                div(class="flex flex-row justify-between") {
                    div(class="w-10") {
                        span(class="relative flex shrink-0 overflow-hidden rounded-md row-span-2 aspect-square h-auto w-10 border-2 border-black") {
                            img(class="aspect-square h-full w-full", src=pic) {}
                        }
                    }
                    a(class="inline-flex items-center justify-center rounded-md text-sm font-medium transition-colors focus:outline-none focus:ring-2 focus:ring-offset0 disabled:opacity-50 disabled:pointer-events-none data-[state=open]:bg-slate-100 bg-floralwhite text-black hover:bg-slate-200 h-10 w-10 border-2 border-black", href="/auth/logout") {
                        Logout {}
                    }
                }
            }
        }
    } else {
        let login_url = format!("/auth/login?redirect_to={}", uri.path());
        view! {
            cx,
            div(class="w-full") {
                a(class="inline-flex items-center justify-center rounded-md text-sm font-medium transition-colors focus:outline-none focus:ring-2 focus:ring-offset0 disabled:opacity-50 disabled:pointer-events-none data-[state=open]:bg-slate-100 bg-floralwhite text-black hover:bg-slate-200 h-10 py-2 px-4 w-full border-2 border-black", href=login_url) {
                    "Sign in"
                }
            }
        }
    }
}

#[derive(Props)]
pub struct SidebarHeaderProps {
    #[prop(default)]
    img_src: Option<String>,
    title: String,
}

#[component]
pub fn RootSidebar<G: Html>(cx: Scope) -> View<G> {
    view! {
        cx,
        div(class="
            hidden w-48 shrink-0 bg-lightviolet bg-pattern-hideout
            lg:flex lg:flex-col lg:justify-between sticky bottom-0 top-0
            max-h-screen space-y-8 overflow-auto border-black p-4
            lg:border-r-2
        ") {
            div(class="space-y-16") {
                SidebarHeader(title="Antonio Pitasi".into(), img_src="static/bulb.webp".into()) {}
                RootSidebarNav {}
            }
            LoginWidget {}
        }
    }
}

#[component]
pub fn SidebarHeader<G: Html>(
    cx: Scope,
    SidebarHeaderProps { img_src, title }: SidebarHeaderProps,
) -> View<G> {
    view! {
        cx,
        div(class="flex h-10 w-full flex-row gap-1 items-center") {
            (if img_src.is_some() {
                let path = img_src.clone().unwrap();
                view!{ cx,
                    div(class="flex h-8 w-8 shrink-0 items-center justify-center rounded-xl") {
                        StaticImg(path=path, alt="".into(), class="w-full h-full object-contain".into()) {}
                    }
                }
            } else {
                view! { cx,
                    div(class="h-8 w-2") {}
                }
            })

            div(class="flex flex-col font-neu text-md font-medium leading-none text-black") {
                span { (title) }
            }
        }
    }
}

#[component]
pub fn RootSidebarNav<G: Html>(cx: Scope) -> View<G> {
    let items = vec![
        ("/", "Home", Some(Home(cx))),
        ("/articles", "Articles", Some(Notebook(cx))),
        ("/uses", "Uses", Some(App(cx))),
    ];

    let nav_items = View::new_fragment(
        items
            .into_iter()
            .map(|item| {
                view! { cx,
                    SidebarNavItem(href=item.0.into(), icon=item.2.unwrap()) {
                        (item.1)
                    }
                }
            })
            .collect::<Vec<_>>(),
    );

    view! { cx,
        SidebarNav { (nav_items) }
    }
}

#[derive(Props)]
pub struct SidebarNavProps<'a, G: Html> {
    children: Children<'a, G>,
}

#[component]
pub fn SidebarNav<'a, G: Html>(cx: Scope<'a>, props: SidebarNavProps<'a, G>) -> View<G> {
    let children = props.children.call(cx);
    view! { cx, ul(class="space-y-4"){(children)} }
}

#[derive(Props)]
pub struct SidebarNavItemProps<'a, G: Html> {
    children: Children<'a, G>,
    #[prop(default)]
    icon: Option<View<G>>,
    href: String,
}

#[component]
pub fn SidebarNavItem<'a, G: Html>(cx: Scope<'a>, props: SidebarNavItemProps<'a, G>) -> View<G> {
    let active = is_active(use_context::<Uri>(cx).path(), &props.href);
    let children = props.children.call(cx);
    let icon = props.icon.map(|icon| {
        view! {cx,
            span(class="mr-2 h-4 w-4") {
                (icon)
            }
        }
    });

    xstyledview! {
        cx,
        style! {
r#"
        a {
            display: flex; 
            flex-direction: row; 
            justify-content: flex-start; 
            align-items: center; 

            padding-top: 0.5rem;
            padding-bottom: 0.5rem; 
            padding-left: 1rem;
            padding-right: 1rem; 

            border-radius: 0.375rem; 
            border-width: 2px; 
            border-color: #000000; 

            --tw-translate-x: -0.125rem;
            --tw-translate-y: -0.125rem;
            transform: translate(var(--tw-translate-x), var(--tw-translate-y));

            width: 100%; 
            height: auto; 

            font-size: 0.875rem;
            line-height: 1.25rem; 
            font-weight: 500; 

            background-color: #ffffff; 

            --tw-shadow: 1px 1px 0px black,2px 2px 0px black;
            box-shadow: var(--tw-shadow);
        }

        a:hover {
            --tw-shadow: none;
            --tw-translate-x: 0;
            --tw-translate-y: 0;
        }

        a[data-active="true"] {
            --tw-translate-x: 0;
            --tw-translate-y: 0;
            background-color: var(--yellow); 
            box-shadow: none;
        }
"#r
        },
        li {
            a(href=props.href, data-active=active) {
                (icon)
                (children)
            }
        }
    }
}

fn is_active(path: &str, href: &str) -> bool {
    match href {
        "/" => path == "/",
        _ => path.starts_with(href),
    }
}

#[derive(Props)]
pub struct SecondarySidebarProps<'a, G: Html> {
    title: String,
    children: Children<'a, G>,
}

#[component]
pub fn SecondarySidebar<'a, G: Html>(
    cx: Scope<'a>,
    props: SecondarySidebarProps<'a, G>,
) -> View<G> {
    let children = props.children.call(cx);
    view! { cx,
        div(class="sticky bottom-0 top-0 max-h-screen w-full space-y-8 overflow-auto border-black p-4 lg:border-r-2 min-h-full shrink-0 bg-pattern-hideout pb-24 lg:block lg:min-h-0 lg:pb-0") {
            div(class="space-y-16") {
                SidebarHeader(title=props.title) {}
                SidebarNav {
                    (children)
                }
            }
        }
    }
}

#[derive(Props)]
pub struct MetaOGImageProps {
    content: String,
    #[prop(default = "1200".to_string())]
    width: String,
    #[prop(default = "630".to_string())]
    height: String,
}

#[component]
pub fn MetaOGImage<G: Html>(
    cx: Scope,
    MetaOGImageProps {
        content,
        width,
        height,
    }: MetaOGImageProps,
) -> View<G> {
    let c = content.clone();
    view! { cx,
        Metatag(name="og:image".into(), attr:property="og:image", attr:content=&c) {}
        Metatag(name="twitter:image:src".into(), attr:content=&content) {}
        Metatag(name="og:image:width".into(), attr:property="og:image:width", attr:content=&width) {}
        Metatag(name="og:image:height".into(), attr:property="og:image:height", attr:content=&height) {}
        Metatag(name="twitter:card".into(), attr:content="summary_large_image") {}
    }
}

#[derive(Props)]
pub struct HeaderProps {
    title: String,
}

#[component]
pub fn Header<G: Html>(cx: Scope, props: HeaderProps) -> View<G> {
    let uri = use_context::<http::Uri>(cx);
    view! {cx,
        header(class="sticky top-0 z-10 flex w-full items-center justify-between gap-2 overflow-hidden border-b-2 border-black bg-yellow px-3 py-3 lg:justify-end lg:gap-4") {
            span(id="header-title", class="line-clamp-1 text-ellipsis font-bold", style="opacity: 0; transform: translateY(30px) translateZ(0px);") {
                (props.title)
            }
            LazyHeartButton(path=format!("/components/like-btn?url={}", uri.path())) {}
        }
        script {(r#"
            var animation = anime({
              targets: '#header-title',
              translateY: 0,
              opacity: 1,
              easing: 'easeInOutSine',
              autoplay: false
            });

            window.addEventListener("scroll", () => {
                const scrollPercent = Math.min(window.scrollY, 200) / 200;
                animation.seek(scrollPercent * animation.duration);
            }, false);
        "#)}
    }
}

#[derive(Props)]
pub struct LazyHeartButtonProps {
    path: String,
}

#[component]
pub fn LazyHeartButton<G: Html>(cx: Scope, props: LazyHeartButtonProps) -> View<G> {
    view! { cx,
        button(
            class="inline-flex items-center justify-center text-sm font-medium transition-colors focus:outline-none focus:ring-2 focus:ring-offset0 disabled:opacity-50 disabled:pointer-events-none bg-transparent hover:bg-slate-100 data-[state=open]:bg-transparent h-9 px-2 rounded-md",
            hx-get=props.path,
            hx-trigger="load",
            hx-target="this",
            hx-swap="outerHTML",
        ) {
            div(class="flex flex-row items-center justify-center gap-2 font-neu text-3xl font-bold") {
                Heart(filled=false) {}
                span { ".." }
            }
        }
    }
}
