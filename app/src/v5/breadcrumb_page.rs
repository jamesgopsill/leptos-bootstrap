use leptos::prelude::*;

use leptos_bootstrap::v5::{Breadcrumb, BreadcrumbItem, Card, CardBody, CardHeader};

const EXAMPLE: &str = "use leptos_bootstrap::v5::{Breadcrumb, BreadcrumbItem};

#[component]
pub fn Component() -> impl IntoView {
    view! {
        <Breadcrumb>
            <BreadcrumbItem>{\"Home\"}</BreadcrumbItem>
            <BreadcrumbItem>{\"Library\"}</BreadcrumbItem>
        </Breadcrumb>
    }
}";

#[component]
pub fn BreadcrumbPage() -> impl IntoView {
    view! {
        <h1 class="mt-3 mb-3">Breadcrumb</h1>
        <Card class="mb-3">
            <CardHeader>Breadcrumb</CardHeader>
            <CardBody>
                <Breadcrumb>
                    <BreadcrumbItem>{"Home"}</BreadcrumbItem>
                    <BreadcrumbItem>{"Library"}</BreadcrumbItem>
                </Breadcrumb>
                <Card class="mt-2">
                    <CardBody>
                        <pre>{EXAMPLE}</pre>
                    </CardBody>
                </Card>
            </CardBody>
        </Card>
    }
}
