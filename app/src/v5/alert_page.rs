use leptos::prelude::*;

use leptos_bootstrap::v5::{Alert, AlertKind, Card, CardBody, CardHeader};

const EXAMPLE: &str = "use leptos_bootstrap::v5::{Alert, AlertItem};

#[component]
pub fn Component() -> impl IntoView {
    let alert = \"A simple primary alert—check it out!\";
    view! {
        <Alert>
            {alert}
        </Alert>
        <Alert kind=AlertKind::Primary>
            {alert}
        </Alert>
        <Alert kind=AlertKind::Secondary>
            {alert}
        </Alert>
        <Alert kind=AlertKind::Success>
            {alert}
        </Alert>
        <Alert kind=AlertKind::Danger>
            {alert}
        </Alert>
        <Alert kind=AlertKind::Warning>
            {alert}
        </Alert>
        <Alert kind=AlertKind::Info>
            {alert}
        </Alert>
        <Alert kind=AlertKind::Light>
            {alert}
        </Alert>
        <Alert kind=AlertKind::Dark>
            {alert}
        </Alert>
    }
}";

#[component]
pub fn AlertPage() -> impl IntoView {
    let alert = "A simple primary alert—check it out!";
    view! {
        <h1 class="mt-3 mb-3">Alert</h1>
        <Card>
            <CardHeader>Alert</CardHeader>
            <CardBody>
                <Alert>{alert}</Alert>
                <Alert kind=AlertKind::Primary>{alert}</Alert>
                <Alert kind=AlertKind::Secondary>{alert}</Alert>
                <Alert kind=AlertKind::Success>{alert}</Alert>
                <Alert kind=AlertKind::Danger>{alert}</Alert>
                <Alert kind=AlertKind::Warning>{alert}</Alert>
                <Alert kind=AlertKind::Info>{alert}</Alert>
                <Alert kind=AlertKind::Light>{alert}</Alert>
                <Alert kind=AlertKind::Dark>{alert}</Alert>
                <Card class="mt-2">
                    <CardBody>
                        <pre>{EXAMPLE}</pre>
                    </CardBody>
                </Card>
            </CardBody>
        </Card>
    }
}
