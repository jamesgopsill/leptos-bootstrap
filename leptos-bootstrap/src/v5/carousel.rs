use leptos::prelude::*;
use uuid::Uuid;

#[component]
pub fn Carousel<'a>(#[prop(optional, into)] class: &'a str, children: Children) -> impl IntoView {
    let class = format!("carousel slide {}", class);
    let id = Uuid::new_v4();
    let hash_id = format!("#{}", id);
    view! {
        <div id=id.to_string() class=class>
            <div class="carousel-inner">{children()}</div>
            <button
                class="carousel-control-prev"
                type="button"
                data-bs-target=hash_id.clone()
                data-bs-slide="prev"
            >
                <span class="carousel-control-prev-icon" aria-hidden="true"></span>
                <span class="visually-hidden">Previous</span>
            </button>
            <button
                class="carousel-control-next"
                type="button"
                data-bs-target=hash_id
                data-bs-slide="next"
            >
                <span class="carousel-control-next-icon" aria-hidden="true"></span>
                <span class="visually-hidden">Next</span>
            </button>
        </div>
    }
}

#[component]
pub fn CarouselItem<'a>(
    #[prop(optional)] active: bool,
    #[prop(optional, into)] class: &'a str,
    children: Children,
) -> impl IntoView {
    let mut class = format!("carousel-item {}", class);
    if active {
        class.push_str(" active");
    }
    view! { <div class=class>{children()}</div> }
}
