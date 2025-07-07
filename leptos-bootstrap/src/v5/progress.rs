use leptos::prelude::*;
use std::fmt;

pub enum ProgressBarKind {
    Default,
    Success,
    Info,
    Warning,
    Danger,
}

impl fmt::Display for ProgressBarKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Default => "",
            Self::Success => "bg-success",
            Self::Info => "bg-info",
            Self::Warning => "bg-warning",
            Self::Danger => "bg-danger",
        };
        write!(f, "{}", s)
    }
}

#[component]
pub fn ProgressBar<'a>(
    percent: f64,
    #[prop(default=ProgressBarKind::Default)] kind: ProgressBarKind,
    #[prop(default = false)] striped: bool,
    #[prop(default = false)] animated: bool,
    #[prop(optional, into)] class: &'a str,
) -> impl IntoView {
    let progress = format!("progress {}", class);
    let mut bar = format!("progress-bar {}", kind);
    if striped {
        bar.push_str(" progress-bar-striped");
    }
    if animated {
        bar.push_str(" progress-bar-animated");
    }
    let style = format!("width: {}%", percent);
    view! {
        <div class=progress>
            <div class=bar style=style></div>
        </div>
    }
}
