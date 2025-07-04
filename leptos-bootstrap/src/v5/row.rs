use leptos::prelude::*;

#[component]
pub fn Row<'a>(#[prop(optional)] class: &'a str, children: Children) -> impl IntoView {
    let class = format!("row {}", class);
    view! {
        <div class=class>
            {children()}
        </div>
    }
}
