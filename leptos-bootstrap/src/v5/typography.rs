use leptos::prelude::*;

#[component]
pub fn Display<'a>(
    size: &'a u8,
    #[prop(optional)] class: &'a str,
    children: Children,
) -> impl IntoView {
    let class = format!("display-{} {}", size, class);
    view! {
        <h1 class=class>
            {children()}
        </h1>
    }
}

#[component]
pub fn Lead<'a>(#[prop(optional)] class: &'a str, children: Children) -> impl IntoView {
    let class = format!("lead {}", class);
    view! {
        <p class=class>
            {children()}
        </p>
    }
}

#[component]
pub fn BlockQuote<'a>(
    #[prop(optional, into)] source: String,
    #[prop(optional)] class: &'a str,
    children: Children,
) -> impl IntoView {
    let class = format!("blockquote {}", class);
    let empty = source.is_empty();
    view! {
        <blockquote class=class>
            {children()}
            <Show when=move || !empty>
                <figcaption class="blockquote-footer">
                    {source.clone()}
                </figcaption>
            </Show>
        </blockquote>
    }
}
