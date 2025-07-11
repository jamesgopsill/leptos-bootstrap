use leptos::prelude::*;
use std::fmt;

#[component]
pub fn Display<'a>(
    size: &'a u8,
    #[prop(optional, into)] class: &'a str,
    children: Children,
) -> impl IntoView {
    let class = format!("display-{} {}", size, class);
    view! { <h1 class=class>{children()}</h1> }
}

#[component]
pub fn Lead<'a>(#[prop(optional, into)] class: &'a str, children: Children) -> impl IntoView {
    let class = format!("lead {}", class);
    view! { <p class=class>{children()}</p> }
}

#[component]
pub fn BlockQuote<'a>(
    #[prop(optional, into)] source: String,
    #[prop(optional, into)] class: &'a str,
    children: Children,
) -> impl IntoView {
    let class = format!("blockquote {}", class);
    let empty = source.is_empty();
    view! {
        <blockquote class=class>
            {children()} <Show when=move || !empty>
                <figcaption class="blockquote-footer">{source.clone()}</figcaption>
            </Show>
        </blockquote>
    }
}

pub enum ImageKind {
    Fluid,
    Thumbnail,
}

impl fmt::Display for ImageKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Fluid => "img-fluid",
            Self::Thumbnail => "img-thumbnail",
        };
        write!(f, "{}", s)
    }
}

#[component]
pub fn Image<'a>(
    #[prop(default = ImageKind::Fluid)] kind: ImageKind,
    #[prop(optional, into)] src: &'a str,
    #[prop(optional, into)] alt: &'a str,
    #[prop(optional, into)] class: &'a str,
) -> impl IntoView {
    let class = format!("{} {}", kind, class);
    view! { <img src=src class=class alt=alt /> }
}
