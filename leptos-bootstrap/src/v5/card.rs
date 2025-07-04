use leptos::prelude::*;

#[component]
pub fn Card<'a>(#[prop(optional)] class: &'a str, children: Children) -> impl IntoView {
    let class = format!("card {}", class);
    view! {
        <div class=class>
            {children()}
        </div>
    }
}

#[component]
pub fn CardBody<'a>(#[prop(optional)] class: &'a str, children: Children) -> impl IntoView {
    let class = format!("card-body {}", class);
    view! {
        <div class=class>
            {children()}
        </div>
    }
}

#[component]
pub fn CardTitle<'a>(#[prop(optional)] class: &'a str, children: Children) -> impl IntoView {
    let class = format!("card-title {}", class);
    view! {
        <div class=class>
            {children()}
        </div>
    }
}

#[component]
pub fn CardSubTitle<'a>(#[prop(optional)] class: &'a str, children: Children) -> impl IntoView {
    let class = format!("card-sub-title {}", class);
    view! {
        <div class=class>
            {children()}
        </div>
    }
}

#[component]
pub fn CardText<'a>(#[prop(optional)] class: &'a str, children: Children) -> impl IntoView {
    let class = format!("card-text {}", class);
    view! {
        <p class=class>
            {children()}
        </p>
    }
}

#[component]
pub fn CardLink<'a>(
    href: &'a str,
    #[prop(optional)] class: &'a str,
    children: Children,
) -> impl IntoView {
    let class = format!("card-link {}", class);
    view! {
        <a class=class href=href>
            {children()}
        </a>
    }
}

#[component]
pub fn CardHeader<'a>(#[prop(optional)] class: &'a str, children: Children) -> impl IntoView {
    let class = format!("card-header {}", class);
    view! {
        <div class=class>
            {children()}
        </div>
    }
}
