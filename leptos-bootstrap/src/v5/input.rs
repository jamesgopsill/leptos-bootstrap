use leptos::prelude::*;
use std::fmt;

#[component]
pub fn FloatingLabel<'a>(
    #[prop(optional, into)] label: &'a str,
    #[prop(optional, into)] class: &'a str,
    children: Children,
) -> impl IntoView {
    let class = format!("form-floating {}", class);
    view! { <div class=class>{children()} <label>{label}</label></div> }
}

pub enum InputKind {
    Text,
    Email,
    DateTimeLocal,
    DateTime,
    Number,
    Password,
}

impl fmt::Display for InputKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Text => "text",
            Self::Email => "email",
            Self::DateTimeLocal => "datetime-local",
            Self::DateTime => "datetime",
            Self::Number => "number",
            Self::Password => "password",
        };
        write!(f, "{}", s)
    }
}

#[component]
pub fn Input<'a>(
    value: RwSignal<String>,
    #[prop(default = InputKind::Text)] kind: InputKind,
    #[prop(optional, into)] placeholder: &'a str,
    #[prop(optional, into)] class: &'a str,
) -> impl IntoView {
    let class = format!("form-control {}", class);
    view! { <input type=kind.to_string() class=class placeholder=placeholder bind:value=value /> }
}
