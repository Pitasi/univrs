use std::{fs::File, io::BufReader};

use axum::http::Uri;
use data_encoding::HEXLOWER;
use leptos::*;
use leptos_meta::{Body, Html, Link, Meta, Style, Title};
use stylist::style;

use crate::{
    hash,
    icons::{Burger, Home, Logout, Pen, SmallX},
    images::StaticImg,
    pages::auth::{AuthContext, User},
    styledview,
};

#[derive(Debug, Clone)]
pub struct UserContext {
    pub user: User,
}

#[component]
pub fn Layout(
    cx: Scope,
    children: ChildrenFn,
    #[prop(into, optional, default = None)] title: Option<String>,
) -> impl IntoView {
    let t = if let Some(title) = title {
        format!("{title} - Antonio Pitasi")
    } else {
        "Antonio Pitasi".to_string()
    };
    view! {
        cx,
        <Html lang="en" class="bg-floralwhite" />
        <Title text=t />
        <Favicon />
        <Meta charset="utf-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1" />
        <MetaDescription />
        <Link rel="preload" href="/static/Inter-VariableFont_slnt,wght.ttf" crossorigin="anonymous" as_="font" type_="font/ttf"/>
        <Link rel="preload" href="/static/DarkerGrotesque-VariableFont_wght.ttf" crossorigin="anonymous" as_="font" type_="font/ttf"/>
        <CssFile path="style.dist.css" />
        <CssFile path="tailwind.css" />
        // script defer data-domain="anto.pt" src="https://plausible.anto.pt/js/plausible.js" {}
        <Body class="flex min-h-screen" attributes=AdditionalAttributes::from(vec![("hx-ext", "loading-states")]) />

        <script src="/static/anime.min.js"/>
        <script src="/static/htmx.min.js" integrity="sha384-L6OqL9pRWyyFU3+/bjdSri+iIphTN/bvYyM37tICVyOJkWZLpP2vGn6VUEXgzg6h" crossorigin="anonymous"/>
        <script src="/static/htmx-loading-states.js"/>

        <div class="flex flex-1 flex-row">
            <MobileNavbar />
            <RootSidebar />
            {children(cx)}
        </div>
    }
}

#[component]
pub fn MetaDescription(
    cx: Scope,
    #[prop(
        into,
        default = "Antonio's personal website, a backend software engineer passionate about distributed systems and clean, maintainable software. Currently working at Qredo, aiming to decentralize the private keys for cryptocurrencies. Founder of the local community, pisa.dev.".to_string()
    )]
    content: String,
) -> impl IntoView {
    view! {
        cx,
        <Meta name="description" content/>
    }
}

#[component]
pub fn MetaOGImage(
    cx: Scope,
    #[prop()] content: String,
    #[prop(into, default = "1200".to_string())] width: String,
    #[prop(into, default = "630".to_string())] height: String,
) -> impl IntoView {
    view! {
        cx,
        <Meta property="og:image" content={content.clone()}/>
        <Meta name="twitter:image:src" content={content}/>
        <Meta property="og:image:width" content={width}/>
        <Meta property="og:image:height" content={height}/>
        <Meta name="twitter:card" content="summary_large_image"/>
    }
}

#[component]
pub fn Favicon(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <Link rel="apple-touch-icon" sizes="180x180" href="/static/apple-touch-icon.png" />
        <Link rel="icon" type_="image/png" sizes="32x32" href="/static/favicon-32x32.png" />
        <Link rel="icon" type_="image/png" sizes="16x16" href="/static/favicon-16x16.png" />
        <Link rel="manifest" href="/static/site.webmanifest" />
        <Link rel="mask-icon" href="/static/safari-pinned-tab.svg" />
        <Meta name="msapplication-TileColor" content="#da532c" />
        <Meta name="theme-color" content="#ffffff" />
    }
}

#[component]
pub fn CssFile(cx: Scope, #[prop(into)] path: String) -> impl IntoView {
    let path = format!("static/{}", path);
    let input = File::open(&path).expect(format!("failed to open file: {}", &path).as_str());
    let reader = BufReader::new(input);
    let digest = hash::sha256_digest(reader).expect("failed to hash css file");
    let file_url = format!("/{}?{}", path, &HEXLOWER.encode(digest.as_ref())[..5]);

    view! {
        cx,
        <Link rel="stylesheet" type_="text/css" href={file_url}/>
    }
}

#[component]
pub fn MobileNavbar(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <nav class="fixed bottom-0 left-0 z-10 flex h-12 w-full flex-row items-center justify-start border-t-2 border-black bg-acid px-2 lg:hidden">
            <button class="inline-flex items-center justify-center rounded-md text-sm font-medium transition-colors focus:outline-none focus:ring-2 focus:ring-offset-0 disabled:opacity-50 disabled:pointer-events-none data-[state=open]:bg-slate-100 h-10 py-2 px-4"
            onClick="document.getElementById('mobile-dialog').dataset.state = 'open';">
                <Burger />
            </button>
        </nav>
        <div id="mobile-dialog" data-state="closed" class="data-[state=closed]:hidden fixed inset-0 z-50 flex items-start justify-center sm:items-center">
            <div class="fixed inset-0 z-50 bg-black/50 backdrop-blur-sm transition-all duration-100 data-[state=closed]:animate-out data-[state=open]:fade-in" data-aria-hidden="true" aria-hidden="true" style="pointer-events: auto;" />
            <div data-state="open" class="fixed z-50 grid w-full gap-4 rounded-b-lg border-black bg-white shadow-neu-3 animate-in data-[state=open]:fade-in-90 data-[state=open]:slide-in-from-bottom-10 sm:rounded-lg sm:zoom-in-90 data-[state=open]:sm:slide-in-from-bottom-0 bottom-0 top-auto border-0 p-0 sm:max-w-full lg:hidden">
                <div class="sticky bottom-0 top-0 max-h-screen overflow-auto p-4 lg:border-r-2 w-full space-y-20 border-t-2 border-black bg-lightviolet bg-pattern-hideout pb-10">
                    <div class="space-y-10">
                        <SidebarHeader img_src="static/bulb.webp" title="Antonio Pitasi" />
                        <RootSidebarNav />
                    </div>
                    <LoginWidget />
                </div>

                <button type="button" class="absolute top-4 right-4 rounded-sm opacity-70 transition-opacity hover:opacity-100 focus:outline-none focus:ring-2 focus:ring-slate-400 focus:ring-offset-2 disabled:pointer-events-none data-[state=open]:bg-slate-100 dark:focus:ring-slate-400 dark:focus:ring-offset-slate-900 dark:data-[state=open]:bg-slate-800"
                    onClick="document.getElementById('mobile-dialog').dataset.state = 'closed';">
                    <SmallX />
                    <span class="sr-only">Close</span>
                </button>
            </div>
        </div>
    }
}

#[component]
pub fn RootSidebar(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <div class="
            hidden w-48 shrink-0 bg-lightviolet bg-pattern-hideout
            lg:flex lg:flex-col lg:justify-between sticky bottom-0 top-0
            max-h-screen space-y-8 overflow-auto border-black p-4
            lg:border-r-2
        ">
            <div class="space-y-8">
                <SidebarHeader title="Antonio Pitasi" img_src="static/bulb.webp" />
                <RootSidebarNav />
            </div>
            <LoginWidget />
        </div>
    }
}

#[component]
pub fn SidebarHeader(
    cx: Scope,
    #[prop(into, optional)] img_src: String,
    #[prop(into)] title: String,
) -> impl IntoView {
    view! {
        cx,
        <header class="flex h-10 w-full flex-row items-center gap-1">
            {if !img_src.is_empty() {
                view! {cx,
                    <div class="flex h-10 w-10 shrink-0 items-center justify-center rounded-xl">
                        <StaticImg path=img_src alt="" class="w-full h-full object-contain" />
                    </div>
                }
            } else {
                view! {cx, <div class="h-10 w-4"/> }
            }}
            <div class="flex flex-col font-neu text-xl font-bold leading-none tracking-tight text-black">
                <span>{title}</span>
            </div>
        </header>
    }
}

#[component]
pub fn RootSidebarNav(cx: Scope) -> impl IntoView {
    struct Item(&'static str, &'static str, Option<View>);
    let items = vec![
        Item("/", "Home", Some(Home(cx).into_view(cx))),
        Item("/articles", "Articles", Some(Pen(cx).into_view(cx))),
    ];

    view! {
        cx,
        <SidebarNav>
            {items.into_iter()
                .map(|item| {
                    view! {
                        cx,
                        <SidebarNavItem href=item.0 icon=item.2>
                            {item.1}
                        </SidebarNavItem>
                    }
                })
                .collect_view(cx)}
        </SidebarNav>
    }
}

#[component]
pub fn SidebarNav(cx: Scope, children: Children) -> impl IntoView {
    view! { cx, <ul class="space-y-4">{children(cx)}</ul> }
}

#[component]
pub fn SidebarNavItem(
    cx: Scope,
    children: Children,
    #[prop(default = None)] icon: Option<View>,
    #[prop(into)] href: String,
) -> impl IntoView {
    let active = is_active(use_context::<Uri>(cx).unwrap().path(), &href);

    styledview! {
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

        a[data-active] {
            --tw-translate-x: 0;
            --tw-translate-y: 0;
            background-color: var(--yellow); 
            box-shadow: none;
        }
"#r
        },
        <li>
            <a href=href data-active=active>
                {
                    icon.map(|icon| {
                        view! {cx,
                            <span class="mr-2 h-4 w-4">
                                {icon}
                            </span>
                        }
                    })
                }
                {children(cx)}
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

#[component]
pub fn LoginWidget(cx: Scope) -> impl IntoView {
    let auth = use_context::<AuthContext>(cx).expect("AuthContext not set");
    if let Some(user) = auth.current_user {
        view! {
            cx,
            <div class="w-full">
                <div class="flex flex-row justify-between">
                    <div class="w-10">
                        <span class="relative flex shrink-0 overflow-hidden rounded-md row-span-2 aspect-square h-auto w-10 border-2 border-black">
                            <img class="aspect-square h-full w-full" src=user.picture.unwrap_or("/static/bulb.webp".to_string()) />
                        </span>
                    </div>
                    <a class="inline-flex items-center justify-center rounded-md text-sm font-medium transition-colors focus:outline-none focus:ring-2 focus:ring-offset0 disabled:opacity-50 disabled:pointer-events-none data-[state=open]:bg-slate-100 bg-floralwhite text-black hover:bg-slate-200 h-10 w-10 border-2 border-black" href="/auth/logout">
                        <Logout />
                    </a>
                </div>
            </div>
        }
    } else {
        view! {
            cx,
            <div class="w-full">
                <a class="inline-flex items-center justify-center rounded-md text-sm font-medium transition-colors focus:outline-none focus:ring-2 focus:ring-offset0 disabled:opacity-50 disabled:pointer-events-none data-[state=open]:bg-slate-100 bg-floralwhite text-black hover:bg-slate-200 h-10 py-2 px-4 w-full border-2 border-black" href="/auth/login">
                    Sign in
                </a>
            </div>
        }
    }
}

#[component]
pub fn SecondarySidebar(
    cx: Scope,
    #[prop(into)] title: String,
    children: Children,
) -> impl IntoView {
    view! {
        cx,
        <div class="sticky bottom-0 top-0 max-h-screen w-full space-y-8 overflow-auto border-black p-4 lg:border-r-2 min-h-full shrink-0 bg-pattern-hideout pb-24 lg:block lg:min-h-0 lg:pb-0">
            <div class="space-y-8">
                <SidebarHeader title />
                <SidebarNav>
                    {children(cx)}
                </SidebarNav>
            </div>
        </div>
    }
}
