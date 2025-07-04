use leptos::prelude::*;

#[component]
pub fn NavLink<'a>(
    href: &'a str,
    #[prop(optional)] active: bool,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] class: &'a str,
    children: Children,
) -> impl IntoView {
    let mut className = "nav-link".to_string();
    if active {
        className.push_str(" active");
    }
    if disabled {
        className.push_str(" disabled");
    }
    let class = format!("nav-link {}", class);
    view! {
        <a
            class=class
            href=href>
            {children()}
        </a>
    }
}

#[component]
pub fn NavBar<'a>(#[prop(optional)] class: &'a str, children: Children) -> impl IntoView {
    let class = format!("navbar {}", class);
    view! {
        <nav class=class>
            {children()}
        </nav>
    }
}

#[component]
pub fn NavBarBrand<'a>(
    href: &'a str,
    #[prop(optional)] class: &'a str,
    children: Children,
) -> impl IntoView {
    let class = format!("navbar-brand {}", class);
    view! {
        <a class=class href=href>
            {children()}
        </a>
    }
}

#[component]
pub fn NavBarMenu(children: Children) -> impl IntoView {
    view! {
        <button
            class="navbar-toggler"
            type="button"
            data-bs-toggle="collapse"
            data-bs-target="#navbarNavAltMarkup"
            aria-controls="navbarNavAltMarkup"
            aria-expanded="false"
            aria-label="Toggle navigation"
        >
            <span class="navbar-toggler-icon"></span>
        </button>
        <div class="collapse navbar-collapse" id="navbarNavAltMarkup">
            {children()}
        </div>
    }
}

#[component]
pub fn NavItem(children: Children) -> impl IntoView {
    view! {
        <div class="navbar-item">
            {children()}
        </div>
    }
}

#[component]
pub fn NavBarNav<'a>(#[prop(optional)] class: &'a str, children: Children) -> impl IntoView {
    let class = format!("navbar-nav {}", class);
    view! {
        <ul class=class>
            {children()}
        </ul>
    }
}

#[component]
pub fn NavBarDropDown<'a>(
    label: &'a str,
    #[prop(optional)] class: &'a str,
    children: Children,
) -> impl IntoView {
    let class = format!("navbar-item dropdown {}", class);
    view! {
        <li class=class>
            <a class="nav-link dropdown-toggle" href="#" role="button" data-bs-toggle="dropdown" aria-expanded="false">
                {label}
            </a>
            <ul class="dropdown-menu">
                {children()}
            </ul>
        </li>
    }
}

#[component]
pub fn NavBarDropDownItem<'a>(
    #[prop(optional)] href: &'a str,
    #[prop(optional)] class: &'a str,
    children: Children,
) -> impl IntoView {
    let class = format!("dropdown-item {}", class);
    if href.is_empty() {
        view! {
            <li class=class>{children()}</li>
        }
        .into_any()
    } else {
        view! {
            <li><a class=class href=href>{children()}</a></li>
        }
        .into_any()
    }
}

#[component]
pub fn NavBarText<'a>(#[prop(optional)] class: &'a str, children: Children) -> impl IntoView {
    let class = format!("navbar-text {}", class);
    view! {
        <span class=class>
            {children()}
        </span>
    }
}
