use leptos::prelude::*;

#[component]
pub fn FormGroup<'a>(
    label: &'a str,
    #[prop(optional, into)] help: &'a str,
    #[prop(optional, into)] class: &'a str,
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
