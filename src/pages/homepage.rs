use axum::{http, response::IntoResponse};
use maud::html;

use crate::{
    components::{root, Meta},
    icons, images, AuthContext,
};

pub async fn handler(uri: http::Uri, auth: AuthContext) -> impl IntoResponse {
    let socials = [
        ("Twitter", icons::twitter(), "https://twitter.com/zaphodias"),
        ("Github", icons::github(), "https://github.com/pitasi"),
        (
            "LinkedIn",
            icons::linkedin(),
            "https://www.linkedin.com/in/pitasi/",
        ),
    ];

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

    root(
        &uri,
        Meta::default(),
        html! {
            main class="mx-auto my-20 max-w-2xl space-y-16 px-6 text-liver lg:px-14" {
                section class="typography" {
                    p {"
I'm Antonio, a backend software engineer. I'm passionate about distributed
systems and clean maintainable software. In my free time, I organize events
with the local community I founded: " a href="https://pisa.dev" { "pisa.dev" } ". "}

                    p {"
I'm currently working on exciting technology at " a href="https://qredo.com" { "Qredo" } ". We aim to decentralize
the private keys for your cryptocurrencies using our dMPC solution. "}

                    p {"
Before that, I worked at " a href="https://ignite.com" { "Ignite" } " (also known as " a href="https://tendermint.com" { "Tendermint" } "), the company that
first created " a href="https://blog.cosmos.network/cosmos-history-inception-to-prelaunch-b05bcb6a4b2b" { "Proof-of-Stake" } " and " a href="https://cosmos.network/" { "Cosmos SDK" } ". My role was Senior Backend
Engineer for the " em { "(now defunct)" } " " a href="https://emeris.com" { "Emeris" } ". "}

                    p {"
Before diving into cryptocurrencies tech, I've cutted my teeth in fast-paced
startups where I helped shaping products such as " a href="https://traent.com" { "Traent" } " and " a href="Zerynth" { "Zerynth" } ". "}

                    p {"
Sometimes I have over-engineering tendencies, such as " a href="https://github.com/Pitasi/univrs" { "my personal website" } ".
Most of the times I'm harmless though. "}
                }

                section {
                    ul class="flex flex-row gap-4" {
                        @for (_, icon, href) in socials.iter() {
                            li {
                                a class="inline-flex items-center justify-center rounded-md text-sm font-medium transition-colors focus:outline-none focus:ring-2 focus:ring-offset0 disabled:opacity-50 disabled:pointer-events-none data-[state=open]:bg-slate-100 h-10 py-2 px-4"
                                    href=(href) {
                                    (icon)
                                }
                            }
                        }
                    }
                }

                section {
                    ol class="mt-6 space-y-6" {
                        @for (name, title, src, from, to) in work_experiences.iter() {
                            li class="flex gap-4" {
                                (images::static_img(src, name, "h-7 w-7 rounded-full"))

                                dl class="flex flex-auto flex-wrap gap-x-2" {
                                    dt class="sr-only" { "Company" }
                                    dd class="w-full flex-none text-sm font-medium text-black" { (name) }

                                    dt class="sr-only" { "Role" }
                                    dd class="text-xs text-eerie" { (title) }

                                    dt class="sr-only" { "Date" }
                                    dd class="ml-auto text-xs text-liver" aria-label=(format!("From {} to {}", from, to)) {
                                        time datetime="2022" { (from) }
                                        " "
                                        span aria-hidden="true" { "—" }
                                        " "
                                        time datetime="Present" { (to) }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        },
        auth.current_user,
    )
}
