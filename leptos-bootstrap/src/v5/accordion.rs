use leptos::prelude::*;
use uuid::Uuid;

#[component]
pub fn Accordion<'a>(#[prop(optional, into)] class: &'a str, children: Children) -> impl IntoView {
    let class = format!("accordion {}", class);
    view! { <div class=class>{children()}</div> }
}

#[component]
pub fn AccordionItem<'a>(
    header: &'a str,
    #[prop(optional)] class: &'a str,
    children: Children,
) -> impl IntoView {
    let class = format!("accordion-item {}", class);
    let id = Uuid::new_v4();
    let hash_id = format!("#{}", id);
    view! {
        <div class=class>
            <h2 class="accordion-header">
                <button
                    class="accordion-button collapsed"
                    type="button"
                    data-bs-toggle="collapse"
                    data-bs-target=hash_id
                >
                    {header}
                </button>
            </h2>
            <div id=id.to_string() class="accordion-collapse collapse">
                <div class="accordion-body">{children()}</div>
            </div>
        </div>
    }
}
