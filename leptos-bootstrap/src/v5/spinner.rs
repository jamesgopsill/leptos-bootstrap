use leptos::prelude::*;
use std::fmt;

pub enum SpinnerKind {
    Default,
    Primary,
    Secondary,
    Success,
    Danger,
    Warning,
    Info,
    Light,
    Dark,
}

impl fmt::Display for SpinnerKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Default => "",
            Self::Primary => "text-primary",
            Self::Secondary => "text-secondary",
            Self::Success => "text-success",
            Self::Danger => "text-danger",
            Self::Warning => "text-warning",
            Self::Info => "text-info",
            Self::Light => "text-light",
            Self::Dark => "text-dark",
        };
        write!(f, "{}", s)
    }
}

pub enum SpinnerSize {
    Small,
    Normal,
    Large,
}

impl fmt::Display for SpinnerSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Small => "spinner-border-sm",
            Self::Normal => "",
            Self::Large => "spinner-border-lg",
        };
        write!(f, "{}", s)
    }
}

#[component]
pub fn Spinner<'a>(
    #[prop(default = SpinnerKind::Default)] kind: SpinnerKind,
    #[prop(default = SpinnerSize::Normal)] size: SpinnerSize,
    #[prop(optional, into)] class: &'a str,
) -> impl IntoView {
    let class = format!("spinner-border {} {} {}", kind, size, class);
    view! {
        <div class=class role="status">
            <span class="visually-hidden">Loading...</span>
        </div>
    }
}

#[component]
pub fn GrowingSpinner<'a>(
    #[prop(default = SpinnerKind::Default)] kind: SpinnerKind,
    #[prop(default = SpinnerSize::Normal)] size: SpinnerSize,
    #[prop(optional, into)] class: &'a str,
) -> impl IntoView {
    let class = format!("spinner-grow {} {} {}", kind, size, class);
    view! {
        <div class=class role="status">
            <span class="visually-hidden">Loading...</span>
        </div>
    }
}
