use leptos::prelude::*;

use leptos_bootstrap::v5::{Card, CardBody, CardHeader};

const EXAMPLE: &str = "use leptos_bootstrap::v5::{Card, CardHeader, CardBody};

#[component]
pub fn Component() -> impl IntoView {
view! {
    <Card>
        <CardHeader>{\"Card Header\"}</CardHeader>
        <CardBody>{\"Card Body\"}</Card>
    </Card>
}";

#[component]
pub fn CardPage() -> impl IntoView {
    view! {
        <h1 class="mt-3 mb-3">Card</h1>
        <Card class="mb-3">
            <CardHeader>Card</CardHeader>
            <CardBody>
                <Card>
                    <CardHeader>{"Card Header"}</CardHeader>
                    <CardBody>{"Card Body"}</CardBody>
                </Card>
                <Card class="mt-2">
                    <CardBody>
                        <pre>{EXAMPLE}</pre>
                    </CardBody>
                </Card>
            </CardBody>
        </Card>
    }
}
