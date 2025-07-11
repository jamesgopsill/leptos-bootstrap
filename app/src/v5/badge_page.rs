use leptos::prelude::*;

use leptos_bootstrap::v5::{Badge, BadgeKind, Card, CardBody, CardHeader};

const EXAMPLE_ONE: &str = "use leptos_bootstrap::v5::{Alert, AlertItem};

#[component]
pub fn Component() -> impl IntoView {
    let heading_text = \"Example heading\";
    view! {
        <h1>{heading_text} <Badge>{\"New\"}</Badge></h1>
        <h2>{heading_text} <Badge>{\"New\"}</Badge></h2>
        <h3>{heading_text} <Badge>{\"New\"}</Badge></h3>
        <h4>{heading_text} <Badge>{\"New\"}</Badge></h4>
        <h5>{heading_text} <Badge>{\"New\"}</Badge></h5>
        <h6>{heading_text} <Badge>{\"New\"}</Badge></h6>
    }
}";

const EXAMPLE_TWO: &str = "use leptos_bootstrap::v5::{Alert, AlertItem};

#[component]
pub fn Component() -> impl IntoView {
    view! {
        <Badge>{\"Default\"}</Badge>
        <Badge kind=BadgeKind::Primary>{\"Primay\"}</Badge>
        <Badge kind=BadgeKind::Secondary>{\"Secondary\"}</Badge>
        <Badge kind=BadgeKind::Success>{\"Success\"}</Badge>
        <Badge kind=BadgeKind::Danger>{\"Danger\"}</Badge>
        <Badge kind=BadgeKind::Warning>{\"Warning\"}</Badge>
        <Badge kind=BadgeKind::Info>{\"Info\"}</Badge>
        <Badge kind=BadgeKind::Light>{\"Light\"}</Badge>
        <Badge kind=BadgeKind::Dark>{\"Dark\"}</Badge>
    }
}";

#[component]
pub fn BadgePage() -> impl IntoView {
    let heading_text = "Example heading";
    view! {
        <h1 class="mt-3 mb-3">Badge</h1>
        <Card class="mb-3">
            <CardHeader>Badge</CardHeader>
            <CardBody>
                <h1>{heading_text} <Badge>{"New"}</Badge></h1>
                <h2>{heading_text} <Badge>{"New"}</Badge></h2>
                <h3>{heading_text} <Badge>{"New"}</Badge></h3>
                <h4>{heading_text} <Badge>{"New"}</Badge></h4>
                <h5>{heading_text} <Badge>{"New"}</Badge></h5>
                <h6>{heading_text} <Badge>{"New"}</Badge></h6>
                <Card class="mt-2">
                    <CardBody>
                        <pre>{EXAMPLE_ONE}</pre>
                    </CardBody>
                </Card>
            </CardBody>
        </Card>
        <Card>
            <CardHeader>{"Badge Kinds"}</CardHeader>
            <CardBody>
                <Badge>{"Default"}</Badge>
                <Badge kind=BadgeKind::Primary>{"Primay"}</Badge>
                <Badge kind=BadgeKind::Secondary>{"Secondary"}</Badge>
                <Badge kind=BadgeKind::Success>{"Success"}</Badge>
                <Badge kind=BadgeKind::Danger>{"Danger"}</Badge>
                <Badge kind=BadgeKind::Warning>{"Warning"}</Badge>
                <Badge kind=BadgeKind::Info>{"Info"}</Badge>
                <Badge kind=BadgeKind::Light>{"Light"}</Badge>
                <Badge kind=BadgeKind::Dark>{"Dark"}</Badge>
                <Card class="mt-2">
                    <CardBody>
                        <pre>{EXAMPLE_TWO}</pre>
                    </CardBody>
                </Card>
            </CardBody>
        </Card>
    }
}
