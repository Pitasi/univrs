use rscx::*;

#[component]
pub fn Twitter() -> String {
    html! {
        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="currentColor" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="inline-block">
            <path d="M22 4s-.7 2.1-2 3.4c1.6 10-9.4 17.3-18 11.6 2.2.1 4.4-.6 6-2C3 15.5.5 9.6 3 5c2.2 2.6 5.6 4.1 9 4-.9-4.2 4-6.6 7-3.8 1.1 0 3-1.2 3-1.2z" />
        </svg>
    }
}

#[component]
pub fn Github() -> String {
    html! {
        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="currentColor" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="inline-block">
            <path d="M15 22v-4a4.8 4.8 0 0 0-1-3.5c3 0 6-2 6-5.5.08-1.25-.27-2.48-1-3.5.28-1.15.28-2.35 0-3.5 0 0-1 0-3 1.5-2.64-.5-5.36-.5-8 0C6 2 5 2 5 2c-.3 1.15-.3 2.35 0 3.5A5.403 5.403 0 0 0 4 9c0 3.5 3 5.5 6 5.5-.39.49-.68 1.05-.85 1.65-.17.6-.22 1.23-.15 1.85v4" />
            <path d="M9 18c-4.51 2-5-2-7-2" />
        </svg>
    }
}

#[component]
pub fn LinkedIn() -> String {
    html! {
        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="currentColor" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="inline-block">
            <path d="M16 8a6 6 0 0 1 6 6v7h-4v-7a2 2 0 0 0-2-2 2 2 0 0 0-2 2v7h-4v-7a6 6 0 0 1 6-6z" />
            <rect x="2" y="9" width="4" height="12" />
            <circle cx="4" cy="4" r="2" />
        </svg>
    }
}

#[component]
pub fn Burger() -> String {
    html! {
        <svg xmlns="http://www.w3.org/2000/svg" width="25" height="25" viewBox="0 0 24 24" fill="currentColor" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-menu">
            <line x1="4" y1="12" x2="20" y2="12" />
            <line x1="4" y1="6" x2="20" y2="6" />
            <line x1="4" y1="18" x2="20" y2="18" />
        </svg>
    }
}

#[component]
pub fn SmallX() -> String {
    html! {
        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="currentColor" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="h-4 w-4">
            <line x1="18" y1="6" x2="6" y2="18" />
            <line x1="6" y1="6" x2="18" y2="18" />
        </svg>
    }
}

#[component]
pub fn Logout() -> String {
    html! {
        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-log-out">
            <path d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4" />
            <polyline points="16 17 21 12 16 7" />
            <line x1="21" y1="12" x2="9" y2="12" />
        </svg>
    }
}

#[component]
pub fn Notebook() -> String {
    html! {
        <svg class="mr-2 h-4 w-4" width="16" height="16" viewBox="0 0 16 16" xmlns="http://www.w3.org/2000/svg">
            <path fill="currentColor" d="M3.75 1A1.75 1.75 0 0 0 2 2.75v10.5c0 .966.784 1.75 1.75 1.75h7.5A1.75 1.75 0 0 0 13 13.25V2.75A1.75 1.75 0 0 0 11.25 1h-7.5ZM4 3.75A.75.75 0 0 1 4.75 3h5.5a.75.75 0 0 1 .75.75v.5a.75.75 0 0 1-.75.75h-5.5A.75.75 0 0 1 4 4.25v-.5ZM14 4h.625c.207 0 .375.168.375.375v1.25a.375.375 0 0 1-.375.375H14V4Zm.625 3H14v2h.625A.375.375 0 0 0 15 8.625v-1.25A.375.375 0 0 0 14.625 7ZM14 10h.625c.207 0 .375.168.375.375v1.25a.375.375 0 0 1-.375.375H14v-2Z" />
        </svg>
    }
}

#[component]
pub fn Home() -> String {
    html! {
        <svg class="mr-2 h-4 w-4" width="24" height="24" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
            <path fill="currentColor" fill-rule="evenodd" d="M12.707 2.293a1 1 0 0 0-1.414 0l-7 7l-2 2a1 1 0 1 0 1.414 1.414L4 12.414V19a3 3 0 0 0 3 3h10a3 3 0 0 0 3-3v-6.586l.293.293a1 1 0 0 0 1.414-1.414l-9-9Z" clip-rule="evenodd" />
        </svg>
    }
}

#[props]
pub struct HeartProps {
    filled: bool,
    #[builder(default)]
    class: Option<String>,
}

#[component]
pub fn Heart(props: HeartProps) -> String {
    let fill = if props.filled { "red" } else { "white" };
    let class = format!("drop-shadow-neu-2 {}", props.class.unwrap_or("".into()));
    html! {
        <svg class=class xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill=fill stroke="black" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path d="M20.42 4.58a5.4 5.4 0 0 0-7.65 0l-.77.78-.77-.78a5.4 5.4 0 0 0-7.65 0C1.46 6.7 1.33 10.28 4 13l8 8 8-8c2.67-2.72 2.54-6.3.42-8.42z" />
        </svg>
    }
}

#[component]
pub fn App() -> String {
    html! {
        <svg class="mr-2 h-4 w-4" width="256" height="256" viewBox="0 0 256 256" xmlns="http://www.w3.org/2000/svg">
            <path fill="currentColor" d="M216 40H40a16 16 0 0 0-16 16v144a16 16 0 0 0 16 16h176a16 16 0 0 0 16-16V56a16 16 0 0 0-16-16ZM68 96a12 12 0 1 1 12-12a12 12 0 0 1-12 12Zm40 0a12 12 0 1 1 12-12a12 12 0 0 1-12 12Z" />
        </svg>
    }
}

#[component]
pub fn Bookmark() -> String {
    html! {
        <svg class="mr-2 h-4 w-4" width="24" height="24" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
            <path fill="currentColor" d="M5 21V5q0-.825.588-1.413T7 3h10q.825 0 1.413.588T19 5v16l-7-3l-7 3Z"/>
        </svg>
    }
}

#[component]
pub fn Link() -> String {
    html! {
        <svg class="h-5 w-5 inline" width="256" height="256" viewBox="0 0 256 256" xmlns="http://www.w3.org/2000/svg">
            <path fill="currentColor" d="M136.37 187.53a12 12 0 0 1 0 17l-5.94 5.94a60 60 0 0 1-84.88-84.88l24.12-24.11A60 60 0 0 1 152 99a12 12 0 1 1-16 18a36 36 0 0 0-49.37 1.47l-24.1 24.08a36 36 0 0 0 50.92 50.92l5.94-5.94a12 12 0 0 1 16.98 0Zm74.08-142a60.09 60.09 0 0 0-84.88 0l-5.94 5.94a12 12 0 0 0 17 17l5.94-5.94a36 36 0 0 1 50.92 50.92l-24.11 24.12A36 36 0 0 1 120 139a12 12 0 1 0-16 18a60 60 0 0 0 82.3-2.43l24.12-24.11a60.09 60.09 0 0 0 .03-84.91Z" />
        </svg>
    }
}
