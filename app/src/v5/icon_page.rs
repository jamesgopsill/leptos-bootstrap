use leptos::prelude::*;

use leptos_bootstrap::v5::{Card, CardBody, CardHeader, Icon, IconKind};

const EXAMPLE: &str = "use leptos_bootstrap::v5::{Icon, IconKind};

#[component]
pub fn Component() -> impl IntoView {
view! {
    <Icon kind=IconKind::Heart />
    <Icon kind=IconKind::HeartFill />
    <Icon kind=IconKind::HeartFill class=\"text-primary\" />
    <Icon kind=IconKind::HeartFill class=\"text-secondary\" />
    <Icon kind=IconKind::HeartFill class=\"text-info\" />
    <Icon kind=IconKind::HeartFill class=\"text-success\" />
    <Icon kind=IconKind::HeartFill class=\"text-warning\" />
    <Icon kind=IconKind::HeartFill class=\"text-danger\" />
}";

#[component]
pub fn IconPage() -> impl IntoView {
    view! {
        <h1 class="mt-3 mb-3">Icons</h1>
        <Card class="mb-3">
            <CardHeader>Icons</CardHeader>
            <CardBody>
                <Icon kind=IconKind::Heart />
                <Icon kind=IconKind::HeartFill />
                <Icon kind=IconKind::HeartFill class="text-primary" />
                <Icon kind=IconKind::HeartFill class="text-secondary" />
                <Icon kind=IconKind::HeartFill class="text-info" />
                <Icon kind=IconKind::HeartFill class="text-success" />
                <Icon kind=IconKind::HeartFill class="text-warning" />
                <Icon kind=IconKind::HeartFill class="text-danger" />
                <Card class="mt-2">
                    <CardBody>
                        <pre>{EXAMPLE}</pre>
                    </CardBody>
                </Card>
            </CardBody>
        </Card>
    }
}
