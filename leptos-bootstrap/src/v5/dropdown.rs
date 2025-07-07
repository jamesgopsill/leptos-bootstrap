use leptos::prelude::*;

use crate::v5::{ButtonKind, ButtonSize};

#[component]
pub fn DropDown<'a>(
    #[prop(optional, into)] label: &'a str,
    #[prop(default = ButtonKind::Primary)] kind: ButtonKind,
    #[prop(optional, into)] class: &'a str,
    children: Children,
) -> impl IntoView {
    let dropdown_class = format!("dropdown {}", class);
    let btn_class = format!("btn dropdown-toggle {}", kind);
    view! {
        <div class=dropdown_class>
            <button class=btn_class type="button" data-bs-toggle="dropdown" aria-expanded="false">
                {label}
            </button>
            <ul class="dropdown-menu">
                {children()}
            </ul>
        </div>
    }
}

#[component]
pub fn DropDownItem<'a>(
    #[prop(default = "#", into)] href: &'a str,
    #[prop(optional, into)] class: &'a str,
    children: Children,
) -> impl IntoView {
    let class = format!("dropdown-item {}", class);
    view! {
        <li>
            <a class=class href=href>{children()}</a>
        </li>
    }
}

#[component]
pub fn DropDownDivider() -> impl IntoView {
    view! {
        <li><hr class="dropdown-divider" /></li>
    }
}

#[component]
pub fn SplitButton<'a>(
    #[prop(optional, into)] label: &'a str,
    #[prop(default = ButtonKind::Primary)] kind: ButtonKind,
    #[prop(default = ButtonSize::Normal)] size: ButtonSize,
    #[prop(optional, into)] class: &'a str,
    children: Children,
) -> impl IntoView {
    let btn_class = format!("btn {} {}", size, class);
    let dropdown_class = format!(
        "btn dropdown-toggle dropdown-toggle-split {} {}",
        kind, size
    );
    view! {
        <div class="btn-group">
            <button type="button" class=btn_class>{label}</button>
            <button type="button" class=dropdown_class data-bs-toggle="dropdown" aria-expanded="false">
                <span class="visually-hidden">Toggle Dropdown</span>
            </button>
            <ul class="dropdown-menu">
                {children()}
            </ul>
        </div>
    }
}
