use leptos::prelude::*;

#[component]
pub fn Breadcrumb<'a>(#[prop(optional, into)] class: &'a str, children: Children) -> impl IntoView {
    let class = format!("breadcrumb {}", class);
    view! {
        <nav aria-label="breadcrumb">
            <ol class=class>
                {children()}
            </ol>
        </nav>
    }
}

#[component]
pub fn BreadcrumbItem<'a>(
    #[prop(default = false)] active: bool,
    #[prop(optional, into)] class: &'a str,
    children: Children,
) -> impl IntoView {
    let mut class = format!("breadcrumb-item {}", class);
    if active {
        class.push_str(" active");
    }
    view! {
        <li class=class>
            {children()}
        </li>
    }
}
