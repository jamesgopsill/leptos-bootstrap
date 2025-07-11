use leptos::prelude::*;

#[component]
pub fn ToastContainer<'a>(
    #[prop(optional, into)] class: &'a str,
    children: Children,
) -> impl IntoView {
    let class = format!("toast-container position-static {}", class);
    view! { <div class=class>{children()}</div> }
}

#[component]
pub fn Toast<'a>(#[prop(optional, into)] class: &'a str, children: Children) -> impl IntoView {
    let class = format!("toast {}", class);
    view! { <div class=class>{children()}</div> }
}

#[component]
pub fn ToastHeader<'a>(
    #[prop(optional, into)] class: &'a str,
    children: Children,
) -> impl IntoView {
    let class = format!("toast-header {}", class);
    view! { <div class=class>{children()}</div> }
}

#[component]
pub fn ToastBody<'a>(#[prop(optional, into)] class: &'a str, children: Children) -> impl IntoView {
    let class = format!("toast-body {}", class);
    view! { <div class=class>{children()}</div> }
}
