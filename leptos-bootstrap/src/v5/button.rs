use leptos::prelude::*;
use std::fmt;

pub enum ButtonKind {
    Primary,
    Secondary,
    Success,
    Danger,
    Warning,
    Info,
    Light,
    Dark,
    Link,
    OutlinePrimary,
    OutlineSecondary,
    OutlineSuccess,
    OutlineDanger,
    OutlineWarning,
    OutlineInfo,
    OutlineLight,
    OutlineDark,
    Close,
}

impl fmt::Display for ButtonKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Primary => "btn-primary",
            Self::Secondary => "btn-secondary",
            Self::Success => "btn-success",
            Self::Danger => "btn-danger",
            Self::Warning => "btn-warning",
            Self::Info => "btn-info",
            Self::Light => "btn-light",
            Self::Dark => "btn-dark",
            Self::Link => "btn-link",
            Self::OutlinePrimary => "btn-outline-primary",
            Self::OutlineSecondary => "btn-outline-secondary",
            Self::OutlineSuccess => "btn-outline-success",
            Self::OutlineDanger => "btn-outline-danger",
            Self::OutlineWarning => "btn-outline-warning",
            Self::OutlineInfo => "btn-outline-info",
            Self::OutlineLight => "btn-outline-light",
            Self::OutlineDark => "btn-outline-dark",
            Self::Close => "btn-close",
        };
        write!(f, "{}", s)
    }
}

pub enum ButtonSize {
    Small,
    Normal,
    Large,
}

impl fmt::Display for ButtonSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Small => "btn-sm",
            Self::Normal => "",
            Self::Large => "btn-lg",
        };
        write!(f, "{}", s)
    }
}

#[component]
pub fn Button<'a>(
    #[prop(default = ButtonKind::Primary)] kind: ButtonKind,
    #[prop(default = ButtonSize::Normal)] size: ButtonSize,
    #[prop(default = false)] disabled: bool,
    #[prop(default = false)] active: bool,
    #[prop(optional, into)] class: &'a str,
    children: Children,
) -> impl IntoView {
    let mut class = format!("btn {} {} {}", kind, size, class.trim());
    if active {
        class.push_str(" active");
    }
    view! {
        <button type="button" class=class disabled=disabled>
            {children()}
        </button>
    }
}

#[component]
pub fn ButtonGroup<'a>(
    #[prop(default = ButtonSize::Normal)] size: ButtonSize,
    #[prop(optional, into)] class: &'a str,
    children: Children,
) -> impl IntoView {
    let class = format!("btn-group {} {}", size, class.trim());
    view! {
        <div class=class role="group">
            {children()}
        </div>
    }
}
