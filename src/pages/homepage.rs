use axum::{http, response::IntoResponse};
use leptos::*;

use crate::{
    components::layout::Layout,
    icons::{Github, LinkedIn, Twitter},
    images::StaticImg,
    leptos::Leptos,
    leptos_root, AuthContext,
};

pub async fn handler(uri: http::Uri, auth: AuthContext) -> impl IntoResponse {
    leptos_root! {
        (uri, auth),
        <Layout>
            <main class="mx-auto my-20 max-w-2xl space-y-16 px-6 text-liver lg:px-14">
                <Intro />
                <Socials />
                <Work />
            </main>
        </Layout>
    }
}

#[component]
fn Intro(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <section class="typography">
            <p>
"I'm Antonio, a backend software engineer. I'm passionate about distributed
systems and clean maintainable software. In my free time, I organize events
with the local community I founded: "<a href="https://pisa.dev">pisa.dev</a>.
            </p>

            <p>
"I'm currently working on exciting technology at "<a href="https://qredo.com">Qredo</a>". We aim to decentralize
the private keys for your cryptocurrencies using our dMPC solution. "
            </p>

            <p>
"Before that, I worked at "<a href="https://ignite.com">Ignite</a>" (also known as "<a href="https://tendermint.com">Tendermint</a>"), the company that
first created "<a href="https://blog.cosmos.network/cosmos-history-inception-to-prelaunch-b05bcb6a4b2b">Proof-of-Stake</a>" and "<a href="https://cosmos.network/">Cosmos SDK</a>". My role was Senior Backend
Engineed for the "<em>(now defunct)</em>" "<a href="https://emeris.com">Emeris</a>". "
            </p>

            <p>
"Before diving into cryptocurrencies tech, I've cutted my teeth in fast-paced
startups where I helped shaping products such as "<a href="https://traent.com">Traent</a>" and "<a href="Zerynth">Zerynth</a>". "
            </p>

            <p>
"Sometimes I have over-engineering tendencies, such as "<a href="https://github.com/Pitasi/univrs">my personal website</a>".
Most of the times I'm harmless though."
            </p>
        </section>
    }
}

#[component]
fn Work(cx: Scope) -> impl IntoView {
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

    view! {
        cx,
        <section>
            <ol class="mt-6 space-y-6">
                {
                    work_experiences.into_iter().map(|item| {
                        view! {
                            cx,
                            <li class="flex gap-4">
                                <StaticImg class="h-7 w-7 rounded-full" path=item.2 alt=item.0 />
                                <dl class="flex flex-auto flex-wrap gap-x-2">
                                    <dt class="sr-only">Company</dt>
                                    <dd class="w-full flex-none text-sm font-medium text-black">{item.0}</dd>

                                    <dt class="sr-only">Role</dt>
                                    <dd class="text-xs text-eerie">{item.1}</dd>

                                    <dt class="sr-only">Date</dt>
                                    <dd class="ml-auto text-xs text-liver" aria-label=format!("From {} to {}", item.3, item.4)>
                                        <time datetime="2022">{item.3}</time>
                                        <span aria-hidden="true">"—"</span>
                                        <time datetime="Present">{item.4}</time>
                                    </dd>
                                </dl>
                            </li>
                        }
                    }).collect_view(cx)
                }
            </ol>
        </section>
    }
}

#[component]
fn Socials(cx: Scope) -> impl IntoView {
    let socials = [
        (
            "Twitter",
            Twitter(cx).into_view(cx),
            "https://twitter.com/zaphodias",
        ),
        (
            "Github",
            Github(cx).into_view(cx),
            "https://github.com/pitasi",
        ),
        (
            "LinkedIn",
            LinkedIn(cx).into_view(cx),
            "https://www.linkedin.com/in/pitasi/",
        ),
    ];

    view! {
        cx,
        <section>
            <ul class="flex flex-row gap-4">
                {
                    socials.into_iter().map(|item| {
                        view! {
                            cx,
                            <li>
                                <a class="inline-flex items-center justify-center rounded-md text-sm font-medium transition-colors focus:outline-none focus:ring-2 focus:ring-offset0 disabled:opacity-50 disabled:pointer-events-none data-[state=open]:bg-slate-100 h-10 py-2 px-4"
                                    href=item.2>
                                    {item.1}
                                </a>
                            </li>
                        }
                    }).collect_view(cx)
                }
            </ul>
        </section>
    }
}
