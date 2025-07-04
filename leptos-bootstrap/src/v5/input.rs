use leptos::prelude::*;

#[component]
pub fn FloatingLabel<'a>(
    #[prop(optional)] label: &'a str,
    #[prop(optional)] class: &'a str,
    children: Children,
) -> impl IntoView {
    let class = format!("form-floating {}", class);
    view! {
        <div class=class>
            {children()}
            <label>{label}</label>
        </div>
    }
}

#[component]
pub fn EmailInput<'a>(
    value: RwSignal<String>,
    #[prop(optional)] placeholder: &'a str,
    #[prop(optional)] class: &'a str,
) -> impl IntoView {
    let class = format!("form-control {}", class);
    view! {
        <input type="email" class=class placeholder=placeholder bind:value=value />
    }
}

#[component]
pub fn TextInput<'a>(
    value: RwSignal<String>,
    #[prop(optional)] placeholder: &'a str,
    #[prop(optional)] class: &'a str,
) -> impl IntoView {
    let class = format!("form-control {}", class);
    view! {
        <input type="text" class=class placeholder=placeholder bind:value=value />
    }
}

#[component]
pub fn DateTimeLocalInput<'a>(
    value: RwSignal<String>,
    #[prop(optional)] placeholder: &'a str,
    #[prop(optional)] class: &'a str,
) -> impl IntoView {
    let class = format!("form-control {}", class);
    view! {
        <input type="datetime-local" class=class placeholder=placeholder bind:value=value />
    }
}

#[component]
pub fn DateInput<'a>(
    value: RwSignal<String>,
    #[prop(optional)] placeholder: &'a str,
    #[prop(optional)] class: &'a str,
) -> impl IntoView {
    let class = format!("form-control {}", class);
    view! {
        <input type="date" class=class placeholder=placeholder bind:value=value />
    }
}

#[component]
pub fn PasswordInput<'a>(
    value: RwSignal<String>,
    #[prop(optional)] placeholder: &'a str,
    #[prop(optional)] class: &'a str,
) -> impl IntoView {
    let class = format!("form-control {}", class);
    view! {
        <input type="password" class=class placeholder=placeholder bind:value=value />
    }
}

#[component]
pub fn NumberInput<'a>(
    value: RwSignal<String>,
    #[prop(optional)] placeholder: &'a str,
    #[prop(optional)] class: &'a str,
) -> impl IntoView {
    let class = format!("form-control {}", class);
    view! {
        <input type="number" class=class placeholder=placeholder bind:value=value />
    }
}
