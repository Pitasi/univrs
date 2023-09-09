use axum::{http, response::IntoResponse};
use rscx::{
    component, context::provide_context, html, props, CollectFragment, CollectFragmentAsync,
    EscapeAttribute,
};

use crate::{
    components::layout::{Layout, LayoutProps},
    icons::{Github, GithubProps, LinkedIn, LinkedInProps, Twitter, TwitterProps},
    images::{StaticImg, StaticImgProps},
    meta::render_with_meta,
    AuthContext,
};

pub async fn handler(uri: http::Uri, auth: AuthContext) -> impl IntoResponse {
    render_with_meta(
        || {
            provide_context(uri);
            provide_context(auth);
        },
        || async {
            html! {
                <Layout>
                    <main class="mx-auto my-20 max-w-2xl space-y-16 px-6 text-liver lg:px-14">
                        <Intro />
                        <Socials />
                        <Work />
                    </main>
                </Layout>
            }
        },
    )
    .await
}

#[component]
fn Intro() -> String {
    html! {
        <section class="typography">
            <p>
                "I'm Antonio, a backend software engineer. I'm passionate about distributed
                systems and clean maintainable software. In my free time, I organize events
                with the local community I founded: "
                <a href="https://pisa.dev">pisa.dev</a>
                "."
            </p>

            <p>
                "I'm currently working on exciting technology at "
                <a href="https://qredo.com">Qredo</a>
                ". We aim to decentralize the private keys for your cryptocurrencies using our dMPC solution. "
            </p>

            <p>
                "Before that, I worked at "
                <a href="https://ignite.com">Ignite</a>
                " (also known as "
                <a href="https://tendermint.com">Tendermint</a>
                "), the company that first created "
                <a href="https://blog.cosmos.network/cosmos-history-inception-to-prelaunch-b05bcb6a4b2b">Proof-of-Stake</a>
                " and "
                <a href="https://cosmos.network/">Cosmos SDK</a>
                ". My role was Senior Backend Engineed for the "
                <em>(now defunct)</em>" "
                <a href="https://emeris.com">Emeris</a>
            </p>

            <p>
                "Before diving into cryptocurrencies tech, I've cutted my teeth in fast-paced
                startups where I helped shaping products such as "
                <a href="https://traent.com">Traent</a>
                " and "
                <a href="https://zerynth.com">Zerynth</a>
                ". "
            </p>

            <p>
                "Sometimes I have over-engineering tendencies, such as "
                <a href="https://github.com/Pitasi/univrs">my personal website</a>
                ". Most of the times I'm harmless though."
            </p>
        </section>
    }
}

#[component]
fn Work() -> String {
    let work_experiences = [
        (
            "Qredo",
            "Blockchain Engineer",
            "static/companies/qredo.webp",
            "2022",
            "Present",
        ),
        (
            "Ignite (fka Tendermint)",
            "Sr. Backend Engineer",
            "static/companies/tendermint.svg",
            "2022",
            "2022",
        ),
        (
            "Geckosoft",
            "Backend Engineer",
            "static/companies/geckosoft.svg",
            "2020",
            "2022",
        ),
        (
            "Nextworks",
            "Backend Engineer",
            "static/companies/nextworks.svg",
            "2019",
            "2020",
        ),
        (
            "Zerynth",
            "Fullstack developer",
            "static/companies/zerynth.svg",
            "2018",
            "2019",
        ),
    ];

    let items = work_experiences.into_iter().map(|(name, title, src, from, to)| async move {
        html! {
            <li class="flex gap-4">
                <StaticImg
                    path=src.into()
                    alt=name.into()
                    class="h-7 w-7 rounded-full".into()
                />

                <dl class="flex flex-auto flex-wrap gap-x-2">
                    <dt class="sr-only">Company</dt>
                    <dd class="w-full flex-none text-sm font-medium text-black">
                        {name}
                    </dd>

                    <dt class="sr-only">Role</dt>
                    <dd class="text-xs text-eerie">
                        {title}
                    </dd>

                    <dt class="sr-only">Date</dt>
                    <dd class="ml-auto text-xs text-liver" aria-label=format!("From {} to {}", from, to)>
                        <time datetime={from}>{from}</time>
                        <span aria-hidden="true">"—"</span>
                        <time datetime={to}>{to}</time>
                    </dd>
                </dl>
            </li>
        }
    }).collect_fragment_async().await;

    html! {
        <section>
            <ol class="mt-6 space-y-6">
                {items}
            </ol>
        </section>
    }
}

#[component]
fn Socials() -> String {
    let socials = [
        (
            "Twitter",
            html! { <Twitter /> },
            "https://twitter.com/zaphodias",
        ),
        ("Github", html! { <Github /> }, "https://github.com/pitasi"),
        (
            "LinkedIn",
            html! { <LinkedIn /> },
            "https://www.linkedin.com/in/pitasi/",
        ),
    ];

    let items = socials.into_iter().map(|item| html! {
        <li>
            <a class="inline-flex items-center justify-center rounded-md text-sm font-medium transition-colors focus:outline-none focus:ring-2 focus:ring-offset0 disabled:opacity-50 disabled:pointer-events-none data-[state=open]:bg-slate-100 h-10 py-2 px-4"
                href=item.2>
                {item.1}
            </a>
        </li>
    }).collect_fragment();

    html! {
        <section>
            <ul class="flex flex-row gap-4">
                {items}
            </ul>
        </section>
    }
}
