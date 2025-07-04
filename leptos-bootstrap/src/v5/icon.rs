use leptos::prelude::*;
use std::fmt;

pub enum IconKind {
    Circle0,
    Circle0Fill,
    BoxArrowRight,
    BoxArrowInRight,
    CupHot,
    CupHotFill,
    Git,
    Github,
    Heart,
    HeartFill,
    HouseFill,
    PersonFill,
    UiChecks,
    TelephoneOutbound,
    FileEarmarkRichtext,
    People,
}

impl fmt::Display for IconKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Circle0 => "bi-0-circle",
            Self::Circle0Fill => "bi-0-circle-fill",
            Self::BoxArrowRight => "bi-box-arrow-right",
            Self::BoxArrowInRight => "bi-box-arrow-in-right",
            Self::CupHot => "bi-cup-hot",
            Self::CupHotFill => "bi-cup-hot-fill",
            Self::Git => "bi-git",
            Self::Github => "bi-github",
            Self::Heart => "bi-heart",
            Self::HeartFill => "bi-heart-fill",
            Self::HouseFill => "bi-house-fill",
            Self::PersonFill => "bi-person-fill",
            Self::UiChecks => "bi-ui-checks",
            Self::TelephoneOutbound => "bi-telephone-outbound",
            Self::FileEarmarkRichtext => "bi-file-earmark-richtext",
            Self::People => "bi-people",
        };
        write!(f, "{}", s)
    }
}

#[component]
pub fn Icon<'a>(kind: IconKind, #[prop(optional)] class: &'a str) -> impl IntoView {
    let class = format!("bi {} {}", kind, class);
    view! {
        <i class=class></i>
    }
}
