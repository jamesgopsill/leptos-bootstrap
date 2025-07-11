use leptos::prelude::*;

#[component]
pub fn Modal<'a>(#[prop(optional, into)] class: &'a str, children: Children) -> impl IntoView {
    let class = format!("modal {}", class);
    view! {
        <div class=class tabindex="-1">
            <div class="modal-dialog">
                <div class="modal-content">{children()}</div>
            </div>
        </div>
    }
}

#[component]
pub fn ModalHeader<'a>(
    #[prop(optional, into)] class: &'a str,
    children: Children,
) -> impl IntoView {
    let class = format!("modal-header {}", class);
    view! {
        <div class=class>
            <h5 class="modal-title">{children()}</h5>
            <button
                type="button"
                class="btn-close"
                data-bs-dismiss="modal"
                aria-label="Close"
            ></button>
        </div>
    }
}

#[component]
pub fn ModalBody<'a>(#[prop(optional, into)] class: &'a str, children: Children) -> impl IntoView {
    let class = format!("modal-body {}", class);
    view! { <div class=class>{children()}</div> }
}

#[component]
pub fn ModalFooter<'a>(
    #[prop(optional, into)] class: &'a str,
    children: Children,
) -> impl IntoView {
    let class = format!("modal-footer {}", class);
    view! { <div class=class>{children()}</div> }
}
