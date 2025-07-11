use leptos::prelude::*;
use std::fmt;

pub enum PaginationSize {
    Small,
    Normal,
    Large,
}

impl fmt::Display for PaginationSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Small => "pagination-sm",
            Self::Normal => "",
            Self::Large => "pagination-lg",
        };
        write!(f, "{}", s)
    }
}

pub enum PaginationAlignment {
    Start,
    Center,
    End,
}

impl fmt::Display for PaginationAlignment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Start => "",
            Self::Center => "justify-content-center",
            Self::End => "justify-content-end",
        };
        write!(f, "{}", s)
    }
}

#[component]
pub fn Pagination<'a>(
    #[prop(default = PaginationSize::Normal)] size: PaginationSize,
    #[prop(default = PaginationAlignment::Start)] align: PaginationAlignment,
    #[prop(optional, into)] class: &'a str,
    children: Children,
) -> impl IntoView {
    let class = format!("pagination {} {} {}", align, size, class);
    view! {
        <nav>
            <ul class=class>{children()}</ul>
        </nav>
    }
}

#[component]
pub fn PageItem<'a>(
    #[prop(default = false)] active: bool,
    #[prop(default = false)] disabled: bool,
    #[prop(optional, into)] href: &'a str,
    children: Children,
) -> impl IntoView {
    let mut class = "page-item".to_string();
    if active {
        class.push_str(" active");
    }
    if disabled {
        class.push_str(" disabled");
    }
    view! {
        <li class=class>
            <a href=href class="page-link">
                {children()}
            </a>
        </li>
    }
}
