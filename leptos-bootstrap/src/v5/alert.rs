use leptos::prelude::*;
use std::fmt;

pub enum AlertKind {
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

impl fmt::Display for AlertKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Primary => "alert-primary",
            Self::Secondary => "alert-secondary",
            Self::Success => "alert-success",
            Self::Danger => "alert-danger",
            Self::Warning => "alert-warning",
            Self::Info => "alert-info",
            Self::Light => "alert-light",
            Self::Dark => "alert-dark",
            Self::Link => "alert-link",
        };
        write!(f, "{}", s)
    }
}

#[component]
pub fn Alert<'a>(
    #[prop(default = AlertKind::Primary)] kind: AlertKind,
    #[prop(optional)] class: &'a str,
    children: Children,
) -> impl IntoView {
    let class = format!("alert {} {}", kind, class);
    view! {
        <div class=class role="alert">
            {children()}
        </div>
    }
}

#[component]
pub fn AlertHeader<'a>(#[prop(optional)] class: &'a str, children: Children) -> impl IntoView {
    let class = format!("alert-heading {}", class);
    view! {
        <h4 class=class>
            {children()}
        </h4>
    }
}
