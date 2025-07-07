use leptos::prelude::*;

#[component]
pub fn Container<'a>(#[prop(optional, into)] class: &'a str, children: Children) -> impl IntoView {
    let class = format!("container {}", class);
    view! {
        <div class=class>
            {children()}
        </div>
    }
}

#[component]
pub fn ContainerFluid<'a>(
    #[prop(optional, into)] class: &'a str,
    children: Children,
) -> impl IntoView {
    let class = format!("container-fluid {}", class);
    view! {
        <div class=class>
            {children()}
        </div>
    }
}
