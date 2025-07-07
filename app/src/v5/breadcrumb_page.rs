use leptos::prelude::*;

use leptos_bootstrap::v5::{Breadcrumb, BreadcrumbItem, Card, CardBody, CardHeader};

#[component]
pub fn BreadcrumbPage() -> impl IntoView {
    view! {
        <h1>Breadcrumb</h1>
        <Card class="mb-3">
            <CardHeader>Breadcrumb</CardHeader>
            <CardBody>
                <Breadcrumb>
                    <BreadcrumbItem>{"Home"}</BreadcrumbItem>
                    <BreadcrumbItem>{"Library"}</BreadcrumbItem>
                </Breadcrumb>
                <Card class="mt-2">
                    <CardBody>
                        <pre>
"use leptos_bootstrap::v5::{Breadcrumb, BreadcrumbItem};

#[component]
pub fn Component() -> impl IntoView {
    view! {
        <Breadcrumb>
            <BreadcrumbItem>{\"Home\"}</BreadcrumbItem>
            <BreadcrumbItem>{\"Library\"}</BreadcrumbItem>
        </Breadcrumb>
    }
}"
                        </pre>
                    </CardBody>
                </Card>
            </CardBody>
        </Card>
    }
}
