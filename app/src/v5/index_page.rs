use leptos::prelude::*;

#[component]
pub fn IndexPage() -> impl IntoView {
    view! {
        <div class="container mt-5 py-5 bg-body-tertiary">
            <h1 class="display-5 fw-bold">{"Leptos Bootstrap"}</h1>
            <p class="fs-4">
                {"A suite of Bootstrap Components for your Leptos Rust applications."}
            </p>
        </div>
    }
}
