use sycamore::prelude::*;

#[component]
pub fn Twitter<G: Html>(cx: Scope) -> View<G> {
    view! {
        cx,
        svg(xmlns="http://www.w3.org/2000/svg", width="24", height="24", viewBox="0 0 24 24", fill="currentColor", stroke="currentColor", stroke-width="2", stroke-linecap="round", stroke-linejoin="round", class="inline-block") {
            path(d="M22 4s-.7 2.1-2 3.4c1.6 10-9.4 17.3-18 11.6 2.2.1 4.4-.6 6-2C3 15.5.5 9.6 3 5c2.2 2.6 5.6 4.1 9 4-.9-4.2 4-6.6 7-3.8 1.1 0 3-1.2 3-1.2z") {}
        }
    }
}

#[component]
pub fn Github<G: Html>(cx: Scope) -> View<G> {
    view! {
        cx,
        svg(xmlns="http://www.w3.org/2000/svg", width="24", height="24", viewBox="0 0 24 24", fill="currentColor", stroke="currentColor", stroke-width="2", stroke-linecap="round", stroke-linejoin="round", class="inline-block") {
            path(d="M15 22v-4a4.8 4.8 0 0 0-1-3.5c3 0 6-2 6-5.5.08-1.25-.27-2.48-1-3.5.28-1.15.28-2.35 0-3.5 0 0-1 0-3 1.5-2.64-.5-5.36-.5-8 0C6 2 5 2 5 2c-.3 1.15-.3 2.35 0 3.5A5.403 5.403 0 0 0 4 9c0 3.5 3 5.5 6 5.5-.39.49-.68 1.05-.85 1.65-.17.6-.22 1.23-.15 1.85v4") {}
            path(d="M9 18c-4.51 2-5-2-7-2"){}
        }
    }
}

#[component]
pub fn LinkedIn<G: Html>(cx: Scope) -> View<G> {
    view! {
        cx,
        svg(xmlns="http://www.w3.org/2000/svg", width="24", height="24", viewBox="0 0 24 24", fill="currentColor", stroke="currentColor", stroke-width="2", stroke-linecap="round", stroke-linejoin="round", class="inline-block") {
            path(d="M16 8a6 6 0 0 1 6 6v7h-4v-7a2 2 0 0 0-2-2 2 2 0 0 0-2 2v7h-4v-7a6 6 0 0 1 6-6z") {}
            rect(x="2", y="9", width="4", height="12") {}
            circle(cx="4", cy="4", r="2") {}
        }
    }
}

#[component]
pub fn Burger<G: Html>(cx: Scope) -> View<G> {
    view! {
        cx,
        svg(xmlns="http://www.w3.org/2000/svg", width="25", height="25", viewBox="0 0 24 24", fill="currentColor", stroke="currentColor", stroke-width="3", stroke-linecap="round", stroke-linejoin="round", class="lucide lucide-menu") {
            line(x1="4", y1="12", x2="20", y2="12") {}
            line(x1="4", y1="6", x2="20", y2="6") {}
            line(x1="4", y1="18", x2="20", y2="18") {}
        }
    }
}

#[component]
pub fn SmallX<G: Html>(cx: Scope) -> View<G> {
    view! {
        cx,
        svg(xmlns="http://www.w3.org/2000/svg", width="24", height="24", viewBox="0 0 24 24", fill="currentColor", stroke="currentColor", stroke-width="2", stroke-linecap="round", stroke-linejoin="round", class="h-4 w-4") {
            line(x1="18", y1="6", x2="6", y2="18") {}
            line(x1="6", y1="6", x2="18", y2="18") {}
        }
    }
}

#[component]
pub fn Logout<G: Html>(cx: Scope) -> View<G> {
    view! {
        cx,
        svg(xmlns="http://www.w3.org/2000/svg", width="24", height="24", viewBox="0 0 24 24", fill="none", stroke="currentColor", stroke-width="2", stroke-linecap="round", stroke-linejoin="round", class="lucide lucide-log-out") {
            path(d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4") {}
            polyline(points="16 17 21 12 16 7") {}
            line(x1="21", y1="12", x2="9", y2="12") {}
        }
    }
}

#[component]
pub fn Pen<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        svg(xmlns="http://www.w3.org/2000/svg", width="24", height="24", viewBox="0 0 24 24", fill="none", stroke="currentColor", stroke-width="2", stroke-linecap="round", stroke-linejoin="round", class="mr-2 h-4 w-4") {
            path(d="m12 19 7-7 3 3-7 7-3-3z") {}
            path(d="m18 13-1.5-7.5L2 2l3.5 14.5L13 18l5-5z") {}
            path(d="m2 2 7.586 7.586") {}
            circle(cx="11", cy="11", r="2") {}
        }
    }
}

#[component]
pub fn Home<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        svg(xmlns="http://www.w3.org/2000/svg", width="24", height="24", viewBox="0 0 24 24", fill="none", stroke="currentColor", stroke-width="2", stroke-linecap="round", stroke-linejoin="round", class="mr-2 h-4 w-4") {
            path(d="m3 9 9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z") {}
            polyline(points="9 22 9 12 15 12 15 22") {}
        }
    }
}

#[derive(Props)]
pub struct HeartProps {
    filled: bool,
}

#[component]
pub fn Heart<G: Html>(cx: Scope, props: HeartProps) -> View<G> {
    let fill = if props.filled { "red" } else { "white" };
    view! { cx,
        svg (xmlns="http://www.w3.org/2000/svg", width="24", height="24", viewBox="0 0 24 24", fill=fill, stroke="black", stroke-width="2", stroke-linecap="round", stroke-linejoin="round", class="drop-shadow-neu-2") {
            path(d="M20.42 4.58a5.4 5.4 0 0 0-7.65 0l-.77.78-.77-.78a5.4 5.4 0 0 0-7.65 0C1.46 6.7 1.33 10.28 4 13l8 8 8-8c2.67-2.72 2.54-6.3.42-8.42z") {}
        }
    }
}

