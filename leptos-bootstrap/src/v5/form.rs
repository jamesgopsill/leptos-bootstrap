use leptos::prelude::*;

#[component]
pub fn FormGroup<'a>(
    label: &'a str,
    #[prop(optional)] help: &'a str,
    #[prop(optional)] class: &'a str,
    children: Children,
) -> impl IntoView {
    view! {
        <div class=class>
            <label class="form-label">{label}</label>
            {children()}
            <div class="form-text">{help}</div>
        </div>
    }
}
