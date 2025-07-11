use leptos::prelude::*;
use std::fmt;

pub enum TableKind {
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

impl fmt::Display for TableKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Default => "",
            Self::Primary => "table-primary",
            Self::Secondary => "table-secondary",
            Self::Success => "table-success",
            Self::Danger => "table-danger",
            Self::Warning => "table-warning",
            Self::Info => "table-info",
            Self::Light => "table-light",
            Self::Dark => "table-dark",
        };
        write!(f, "{}", s)
    }
}

pub enum TableBorderKind {
    Default,
    Bordered,
    Borderless,
}

impl fmt::Display for TableBorderKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Default => "",
            Self::Bordered => "table-bordered",
            Self::Borderless => "table-bordeless",
        };
        write!(f, "{}", s)
    }
}

pub enum TableStripeKind {
    Default,
    Rows,
    Cols,
    RowsAndCols,
}

impl fmt::Display for TableStripeKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Default => "",
            Self::Rows => "table-striped",
            Self::Cols => "table-striped-columns",
            Self::RowsAndCols => "table-striped table-striped-columns",
        };
        write!(f, "{}", s)
    }
}

#[component]
pub fn Table<'a>(
    #[prop(default = TableKind::Default)] kind: TableKind,
    #[prop(default = TableBorderKind::Default)] border: TableBorderKind,
    #[prop(default = TableStripeKind::Default)] stripe: TableStripeKind,
    #[prop(default = false)] sm: bool,
    #[prop(default = false)] hover: bool,
    #[prop(optional, into)] class: &'a str,
    children: Children,
) -> impl IntoView {
    let mut class = format!("table {} {} {} {}", kind, border, stripe, class);
    if hover {
        class.push_str(" table-hover");
    }
    if sm {
        class.push_str(" table-sm");
    }
    view! { <table class=class>{children()}</table> }
}

#[component]
pub fn TableHead<'a>(#[prop(optional, into)] class: &'a str, children: Children) -> impl IntoView {
    view! { <thead class=class>{children()}</thead> }
}

#[component]
pub fn TableRow<'a>(#[prop(optional, into)] class: &'a str, children: Children) -> impl IntoView {
    view! { <tr class=class>{children()}</tr> }
}

#[component]
pub fn TableHeadCol<'a>(
    #[prop(optional, into)] class: &'a str,
    children: Children,
) -> impl IntoView {
    view! { <th class=class>{children()}</th> }
}

#[component]
pub fn TableCol<'a>(#[prop(optional, into)] class: &'a str, children: Children) -> impl IntoView {
    view! { <td class=class>{children()}</td> }
}
