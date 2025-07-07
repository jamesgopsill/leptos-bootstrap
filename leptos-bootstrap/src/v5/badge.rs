use leptos::prelude::*;
use std::fmt;

pub enum BadgeKind {
    Primary,
    Secondary,
    Success,
    Danger,
    Warning,
    Info,
    Light,
    Dark,
    Link,
}

impl fmt::Display for BadgeKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Primary => "text-bg-primary",
            Self::Secondary => "text-bg-secondary",
            Self::Success => "text-bg-success",
            Self::Danger => "text-bg-danger",
            Self::Warning => "text-bg-warning",
            Self::Info => "text-bg-info",
            Self::Light => "text-bg-light",
            Self::Dark => "text-bg-dark",
            Self::Link => "text-bg-link",
        };
        write!(f, "{}", s)
    }
}

#[component]
pub fn Badge<'a>(
    #[prop(default = BadgeKind::Primary)] kind: BadgeKind,
    #[prop(default = false)] pill: bool,
    #[prop(optional, into)] class: &'a str,
    children: Children,
) -> impl IntoView {
    let mut class = format!("badge {} {}", kind, class);
    if pill {
        class.push_str(" rounded-pill");
    }
    view! {
        <span class=class>
            {children()}
        </span>
    }
}
