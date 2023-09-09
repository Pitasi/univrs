use std::{fs::File, io::BufReader};

use axum::http::{self, Uri};
use data_encoding::HEXLOWER;
use rscx::{
    component, context::expect_context, html, props, CollectFragmentAsync, EscapeAttribute,
};
use stylist::style;

use crate::{
    hash,
    icons::{
        App, AppProps, Burger, BurgerProps, Heart, HeartProps, Home, HomeProps, Logout,
        LogoutProps, Notebook, NotebookProps, SmallX, SmallXProps,
    },
    images::{StaticImg, StaticImgProps},
    meta::{Dedup, DedupProps, MetaContextRender, MetaContextRenderProps},
    pages::auth::AuthContext,
};

#[props]
pub struct LayoutProps {
    children: String,
    #[builder(default, setter(transform = |t: impl Into<String>| Some(t.into())))]
    title: Option<String>,
    #[builder(default, setter(transform = |t: impl Into<String>| Some(t.into())))]
    description: Option<String>,
    #[builder(default)]
    head: String,
}

#[component]
pub async fn Layout(props: LayoutProps) -> String {
    // render app that populates meta_cx
    let app = html! {
        <script src="/static/anime.min.js"></script>
        <script src="/static/htmx.min.js" integrity="sha384-L6OqL9pRWyyFU3+/bjdSri+iIphTN/bvYyM37tICVyOJkWZLpP2vGn6VUEXgzg6h" crossorigin="anonymous" defer=true></script>
        <script src="/static/htmx-loading-states.js" defer=true></script>

        <div class="flex flex-1 flex-row">
            <MobileNavbar />
            <RootSidebar />
            {props.children}
        </div>
    };

    // render shell
    html! {
        <!DOCTYPE html>
        <html lang="en" class="bg-floralwhite">
            <head>
                <title>{props.title.unwrap_or("Antonio Pitasi".to_string()) }</title>
                <Favicon />
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <meta name="description" content={props.description.unwrap_or("Antonio's personal website, a backend software engineer passionate about clean, maintainable software. Currently working at Qredo, aiming to decentralize the private keys for cryptocurrencies. Founder of the local community, pisa.dev.".to_string()) } />
                <link rel="preload" href="/static/Inter-VariableFont_slnt,wght.ttf" crossorigin="anonymous" as_="font" type="font/ttf" />
                <link rel="preload" href="/static/ClashDisplay-Variable.woff2" crossorigin="anonymous" as_="font" type="font/woff2" />
                <CssFile path="style.dist.css" />
                <CssFile path="tailwind.css" />
                <script defer=true data-domain="anto.pt" src="https://plausible.anto.pt/js/plausible.js"></script>
                <MetaContextRender />
                {props.head}
            </head>

            <body class="flex min-h-screen" hx-ext="loading-states" hx-boost="true">
                {app}
            </body>
        </html>
    }
}

#[component]
pub fn Favicon() -> String {
    html! {
        <link rel="apple-touch-icon" sizes="180x180" href="/static/apple-touch-icon.png" />
        <link rel="icon" type="image/png" sizes="32x32" href="/static/favicon-32x32.png" />
        <link rel="icon" type="image/png" sizes="16x16" href="/static/favicon-16x16.png" />
        <link rel="manifest" href="/static/site.webmanifest" />
        <link rel="mask-icon" href="/static/safari-pinned-tab.svg" />
        <meta name="msapplication-TileColor" content="#da532c" />
        <meta name="theme-color" content="#ffffff" />
    }
}

#[props]
pub struct CssFileProps {
    #[builder(setter(into))]
    path: String,
}

#[component]
pub fn CssFile(props: CssFileProps) -> String {
    let path = format!("static/{}", props.path);
    let input = File::open(&path).expect(format!("failed to open file: {}", &path).as_str());
    let reader = BufReader::new(input);
    let digest = hash::sha256_digest(reader).expect("failed to hash css file");
    let file_url = format!("/{}?{}", path, &HEXLOWER.encode(digest.as_ref())[..5]);

    html! {
        <link rel="stylesheet" type="text/css" href=file_url />
    }
}

#[component]
pub async fn MobileNavbar() -> String {
    html! {
        <nav class="fixed bottom-0 left-0 z-10 flex h-12 w-full flex-row items-center justify-start border-t-2 border-black bg-acid px-2 lg:hidden">
            <button class="inline-flex items-center justify-center rounded-md text-sm font-medium transition-colors focus:outline-none focus:ring-2 focus:ring-offset-0 disabled:opacity-50 disabled:pointer-events-none data-[state=open]:bg-slate-100 h-10 py-2 px-4" onClick="document.getElementById('mobile-dialog').dataset.state = 'open';">
                <Burger />
            </button>
        </nav>

        <div id="mobile-dialog" data-state="closed" class="data-[state=closed]:hidden fixed inset-0 z-50 flex items-start justify-center sm:items-center">
            <div class="fixed inset-0 z-50 bg-black/50 backdrop-blur-sm transition-all duration-100 data-[state=closed]:animate-out data-[state=open]:fade-in"></div>
            <div class="fixed z-50 grid w-full gap-4 rounded-b-lg border-black bg-white shadow-neu-3 animate-in data-[state=open]:fade-in-90 data-[state=open]:slide-in-from-bottom-10 sm:rounded-lg sm:zoom-in-90 data-[state=open]:sm:slide-in-from-bottom-0 bottom-0 top-auto border-0 p-0 sm:max-w-full lg:hidden">
                <div class="sticky bottom-0 top-0 max-h-screen overflow-auto p-4 lg:border-r-2 w-full space-y-20 border-t-2 border-black bg-lightviolet bg-pattern-hideout pb-10">
                    <div class="space-y-8">
                        <SidebarHeader img_src=Some("static/bulb.webp".into()) title="Antonio Pitasi".into() />
                        <RootSidebarNav />
                    </div>
                    <LoginWidget />
                </div>

                <button type="button" class="absolute top-4 right-4 rounded-sm opacity-70 transition-opacity hover:opacity-100 focus:outline-none focus:ring-2 focus:ring-slate-400 focus:ring-offset-2 disabled:pointer-events-none data-[state=open]:bg-slate-100 dark:focus:ring-slate-400 dark:focus:ring-offset-slate-900 dark:data-[state=open]:bg-slate-800" onClick="document.getElementById('mobile-dialog').dataset.state = 'closed';">
                    <SmallX />
                    <span class="sr-only">Close</span>
                </button>
            </div>
        </div>
    }
}

#[component]
pub fn LoginWidget() -> String {
    let uri = expect_context::<http::Uri>();
    let auth = expect_context::<AuthContext>();

    if let Some(user) = &auth.current_user {
        let pic = user
            .picture
            .clone()
            .unwrap_or("/static/bulb.webp".to_string());

        html! {
            <div class="w-full">
                <div class="flex flex-row justify-between">
                    <div class="w-10">
                        <span class="relative flex shrink-0 overflow-hidden rounded-md row-span-2 aspect-square h-auto w-10 border-2 border-black">
                            <img class="aspect-square h-full w-full" src=pic />
                        </span>
                    </div>
                    <a class="inline-flex items-center justify-center rounded-md text-sm font-medium transition-colors focus:outline-none focus:ring-2 focus:ring-offset0 disabled:opacity-50 disabled:pointer-events-none data-[state=open]:bg-slate-100 bg-floralwhite text-black hover:bg-slate-200 h-10 w-10 border-2 border-black" href="/auth/logout">
                        <Logout />
                    </a>
                </div>
            </div>
        }
    } else {
        let login_url = format!("/auth/login?redirect_to={}", uri.path());
        html! {

            <div class="w-full">
                <a class="inline-flex items-center justify-center rounded-md text-sm font-medium transition-colors focus:outline-none focus:ring-2 focus:ring-offset0 disabled:opacity-50 disabled:pointer-events-none data-[state=open]:bg-slate-100 bg-floralwhite text-black hover:bg-slate-200 h-10 py-2 px-4 w-full border-2 border-black" href=login_url hx-boost="false">
                    Sign in
                </a>
            </div>
        }
    }
}

#[props]
pub struct SidebarHeaderProps {
    #[builder(default)]
    img_src: Option<String>,
    title: String,
}

#[component]
pub fn RootSidebar() -> String {
    html! {
        <div class="
            hidden w-48 shrink-0 bg-lightviolet bg-pattern-hideout
            lg:flex lg:flex-col lg:justify-between sticky bottom-0 top-0
            max-h-screen space-y-8 overflow-auto border-black p-4
            lg:border-r-2
        ">
            <div class="space-y-16">
                <SidebarHeader title="Antonio Pitasi".into() img_src=Some("static/bulb.webp".into()) />
                <RootSidebarNav />
            </div>
            <LoginWidget />
        </div>
    }
}

#[component]
pub fn SidebarHeader(SidebarHeaderProps { img_src, title }: SidebarHeaderProps) -> String {
    html! {
        <div class="flex h-10 w-full flex-row gap-1 items-center">
            { if img_src.is_some() {
                let path = img_src.clone().unwrap();
                html!{
                    <div class="flex h-8 w-8 shrink-0 items-center justify-center rounded-xl">
                        <StaticImg path=path alt="".into() class="w-full h-full object-contain".into() />
                    </div>
                }
            } else {
                html! {
                    <div class="h-8 w-2" />
                }
            } }

            <div class="flex flex-col font-neu text-md font-medium leading-none text-black">
                <span>{title}</span>
            </div>
        </div>
    }
}

#[component]
pub fn RootSidebarNav() -> String {
    let items = vec![
        ("/", "Home", Some(html! { <Home /> })),
        ("/articles", "Articles", Some(html! { <Notebook /> })),
        ("/uses", "Uses", Some(html! { <App /> })),
    ];

    let nav_items = items
        .into_iter()
        .map(|item| async move {
            html! {
                <SidebarNavItem href=item.0.into() icon=item.2>
                    {item.1}
                </SidebarNavItem>
            }
        })
        .collect_fragment_async()
        .await;

    html! {
        <SidebarNav>{nav_items}</SidebarNav>
    }
}

#[props]
pub struct SidebarNavProps {
    children: String,
}

#[component]
pub fn SidebarNav(props: SidebarNavProps) -> String {
    html! {
        <ul class="space-y-4">
            {props.children}
        </ul>
    }
}

#[props]
pub struct SidebarNavItemProps {
    children: String,
    #[builder(default)]
    icon: Option<String>,
    href: String,
}

#[component]
pub fn SidebarNavItem(props: SidebarNavItemProps) -> String {
    let active = is_active(expect_context::<Uri>().path(), &props.href);
    let icon = props.icon.map(|icon| {
        html! {
            <span class="mr-2 h-4 w-4">
                {icon}
            </span>
        }
    });

    let (class, style) = {
        let css = style! { r#"
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
            }"#
        }
        .expect("invalid CSS");
        let class = css.get_class_name().to_string();
        let style = css.get_style_str().to_string();
        (class, style)
    };

    html! {
        <Dedup id=class.clone()>
            <style>{style}</style>
        </Dedup>
        <li class=class>
            <a href=props.href data-active=active>
                {icon.unwrap_or(String::new())}
                {props.children}
            </a>
        </li>
    }
}

fn is_active(path: &str, href: &str) -> bool {
    match href {
        "/" => path == "/",
        _ => path.starts_with(href),
    }
}

#[props]
pub struct SecondarySidebarProps {
    title: String,
    children: String,
}

#[component]
pub fn SecondarySidebar(props: SecondarySidebarProps) -> String {
    html! {
        <div class="sticky bottom-0 top-0 max-h-screen w-full space-y-8 overflow-auto border-black p-4 lg:border-r-2 min-h-full shrink-0 bg-pattern-hideout pb-24 lg:block lg:min-h-0 lg:pb-0">
            <div class="space-y-16">
                <SidebarHeader title=props.title />
                <SidebarNav>
                    {props.children}
                </SidebarNav>
            </div>
        </div>
    }
}

#[props]
pub struct MetaOGImageProps {
    content: String,
    #[builder(default = "1200".to_string())]
    width: String,
    #[builder(default = "630".to_string())]
    height: String,
}

#[component]
pub fn MetaOGImage(
    MetaOGImageProps {
        content,
        width,
        height,
    }: MetaOGImageProps,
) -> String {
    let c = content.clone();
    html! {
        <meta name="og:image" property="og:image" content=&c />
        <meta name="twitter:image:src" content=&content />
        <meta name="og:image:width" property="og:image:width" content=&width />
        <meta name="og:image:height" property="og:image:height" content=&height />
        <meta name="twitter:card" content="summary_large_image" />
    }
}

#[props]
pub struct HeaderProps {
    title: String,
}

#[component]
pub fn Header(props: HeaderProps) -> String {
    let uri = expect_context::<http::Uri>();
    html! {
        <header class="sticky top-0 z-10 flex w-full items-center justify-between gap-2 overflow-hidden border-b-2 border-black bg-yellow px-3 py-3 lg:justify-end lg:gap-4">
            <span id="header-title" class="line-clamp-1 text-ellipsis font-bold" style="opacity: 0; transform: translateY(30px) translateZ(0px);">
                {props.title}
            </span>
            <LazyHeartButton path=format!("/components/like-btn?url={}", uri.path()) />
        </header>
        <script>{r#"
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
        "#}</script>
    }
}

#[props]
pub struct LazyHeartButtonProps {
    path: String,
}

#[component]
pub fn LazyHeartButton(props: LazyHeartButtonProps) -> String {
    html! {
        <button
            class="inline-flex items-center justify-center text-sm font-medium transition-colors focus:outline-none focus:ring-2 focus:ring-offset0 disabled:opacity-50 disabled:pointer-events-none bg-transparent hover:bg-slate-100 data-[state=open]:bg-transparent h-9 px-2 rounded-md"
            hx-get=props.path
            hx-trigger="load"
            hx-target="this"
            hx-swap="outerHTML"
        >
            <div class="flex flex-row items-center justify-center gap-2 font-neu text-3xl font-bold">
                <Heart filled=false />
                <span>..</span>
            </div>
        </button>
    }
}
