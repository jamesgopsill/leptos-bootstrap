use leptos::prelude::*;

#[component]
pub fn Figure<'a>(
    #[prop(optional, into)] caption: &'a str,
    #[prop(optional, into)] class: &'a str,
    children: Children,
) -> impl IntoView {
    let class = format!("figcaption {}", class.trim());
    view! {
        <figure class=class>
            {children()} <figcaption class="figure-caption">{caption}</figcaption>
        </figure>
    }
}
