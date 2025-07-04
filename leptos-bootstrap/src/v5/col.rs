use leptos::prelude::*;

#[component]
pub fn Col<'a>(
    #[prop(optional, into)] xs: u8,
    #[prop(optional, into)] sm: u8,
    #[prop(optional, into)] md: u8,
    #[prop(optional, into)] lg: u8,
    #[prop(optional, into)] xl: u8,
    #[prop(optional, into)] xxl: u8,
    #[prop(optional, into)] class: &'a str,
    children: Children,
) -> impl IntoView {
    let mut combined_class = "".to_string();
    if xs > 0 {
        combined_class.push_str(&format!(" col-xs-{}", sm));
    }
    if sm > 0 {
        combined_class.push_str(&format!(" col-sm-{}", sm));
    }
    if md > 0 {
        combined_class.push_str(&format!(" col-md-{}", md));
    }
    if lg > 0 {
        combined_class.push_str(&format!(" col-lg-{}", lg));
    }
    if xl > 0 {
        combined_class.push_str(&format!(" col-xl-{}", lg));
    }
    if xxl > 0 {
        combined_class.push_str(&format!(" col-xxl-{}", lg));
    }
    if !class.is_empty() {
        combined_class.push(' ');
        combined_class.push_str(class.trim());
    }
    view! {
        <div class=combined_class>
            {children()}
        </div>
    }
}
