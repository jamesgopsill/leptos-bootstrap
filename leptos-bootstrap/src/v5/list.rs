use leptos::prelude::*;
use std::fmt;

#[component]
pub fn ListGroup<'a>(
    #[prop(default = false)] numbered: bool,
    #[prop(default = false)] flush: bool,
    #[prop(default = false)] horizontal: bool,
    #[prop(optional, into)] class: &'a str,
    children: Children,
) -> impl IntoView {
    let mut class = format!("list-group {}", class);
    if flush {
        class.push_str(" list-group-flush");
    }
    if numbered {
        class.push_str(" list-group-numbered");
    }
    if horizontal {
        class.push_str(" list-group-horizontal");
    }
    view! { <ul class=class>{children()}</ul> }
}

pub enum ListGroupItemKind {
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

impl fmt::Display for ListGroupItemKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Default => "",
            Self::Primary => "list-group-item-primary",
            Self::Secondary => "list-group-item-secondary",
            Self::Success => "list-group-item-success",
            Self::Danger => "list-group-item-danger",
            Self::Warning => "list-group-item-warning",
            Self::Info => "list-group-item-info",
            Self::Light => "list-group-item-light",
            Self::Dark => "list-group-item-dark",
        };
        write!(f, "{}", s)
    }
}

#[component]
pub fn ListGroupItem<'a>(
    #[prop(default = ListGroupItemKind::Default)] kind: ListGroupItemKind,
    #[prop(default = false)] active: bool,
    #[prop(optional, into)] class: &'a str,
    children: Children,
) -> impl IntoView {
    let mut class = format!("list-group-item {} {}", kind, class);
    if active {
        class.push_str(" active");
    }
    view! { <li class=class>{children()}</li> }
}
