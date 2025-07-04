use leptos::prelude::*;

use leptos_bootstrap::v5::{Button, ButtonKind, ButtonSize, Card, CardBody, CardHeader};

#[component]
pub fn ButtonPage() -> impl IntoView {
    view! {
        <h1>Buttons</h1>
        <Card>
            <CardHeader>Variants</CardHeader>
            <CardBody>
                <Button class="me-1">Primary</Button>
                <Button kind=ButtonKind::Secondary class="me-1">Secondary</Button>
                <Button kind=ButtonKind::Success class="me-1">Success</Button>
                <Button kind=ButtonKind::Danger class="me-1">Danger</Button>
                <Button kind=ButtonKind::Warning class="me-1">Warning</Button>
                <Button kind=ButtonKind::Info class="me-1">Info</Button>
                <Button kind=ButtonKind::Light class="me-1">Light</Button>
                <Button kind=ButtonKind::Dark class="me-1">Dark</Button>
                <Button kind=ButtonKind::Link>Link</Button>
                <Card class="mt-2">
                    <CardBody>
                        <pre>
"use leptos_bootstrap::v5::{Button, ButtonKind};

<Button>Primary</Button>
<Button kind=ButtonKind::Secondary>Secondary</Button>
<Button kind=ButtonKind::Success>Success</Button>
<Button kind=ButtonKind::Danger>Danger</Button>
<Button kind=ButtonKind::Warning>Warning</Button>
<Button kind=ButtonKind::Info>Info</Button>
<Button kind=ButtonKind::Light>Light</Button>
<Button kind=ButtonKind::Dark>Dark</Button>
<Button kind=ButtonKind::Link>Link</Button>"
                        </pre>
                    </CardBody>
                </Card>
            </CardBody>
        </Card>
        <Card class="mt-4">
            <CardHeader>Outline Variants</CardHeader>
            <CardBody>
                <Button kind=ButtonKind::OutlinePrimary class="me-1">Primary</Button>
                <Button kind=ButtonKind::OutlineSecondary class="me-1">Secondary</Button>
                <Button kind=ButtonKind::OutlineSuccess class="me-1">Success</Button>
                <Button kind=ButtonKind::OutlineDanger class="me-1">Danger</Button>
                <Button kind=ButtonKind::OutlineWarning class="me-1">Warning</Button>
                <Button kind=ButtonKind::OutlineInfo class="me-1">Info</Button>
                <Button kind=ButtonKind::OutlineLight class="me-1">Light</Button>
                <Button kind=ButtonKind::OutlineDark class="me-1">Dark</Button>
                <Card class="mt-2">
                    <CardBody>
                        <pre>
"use leptos_bootstrap::v5::{Button, ButtonKind};

<Button kind=ButtonKind::OutlinePrimary >Primary</Button>
<Button kind=ButtonKind::OutlineSecondary>Secondary</Button>
<Button kind=ButtonKind::OutlineSuccess>Success</Button>
<Button kind=ButtonKind::OutlineDanger>Danger</Button>
<Button kind=ButtonKind::OutlineWarning>Warning</Button>
<Button kind=ButtonKind::OutlineInfo>Info</Button>
<Button kind=ButtonKind::OutlineLight>Light</Button>
<Button kind=ButtonKind::OutlineDark>Dark</Button>"
                        </pre>
                    </CardBody>
                </Card>
            </CardBody>
        </Card>
        <Card class="mt-4">
            <CardHeader>Sizes</CardHeader>
            <CardBody>
                <Button size=ButtonSize::Large class="me-1">Large</Button>
                <Button class="me-1">Normal</Button>
                <Button size=ButtonSize::Small>Small</Button>
                <Card class="mt-2">
                    <CardBody>
                        <pre>
"use leptos_bootstrap::v5::{Button, ButtonSize};

<Button size=ButtonSize::Large>Large</Button>
<Button>Normal</Button>
<Button size=ButtonSize::Small>Small</Button>"
                        </pre>
                    </CardBody>
                </Card>
            </CardBody>
        </Card>
        <Card class="mt-4">
            <CardHeader>Disable</CardHeader>
            <CardBody>
                <Button class="me-1">Enabled</Button>
                <Button disabled=true>Disabled</Button>
                <Card class="mt-2">
                    <CardBody>
                        <pre>
"use leptos_bootstrap::v5::Button;

<Button>Enabled</Button>
<Button disabled=true>Disabled</Button>"
                        </pre>
                    </CardBody>
                </Card>
            </CardBody>
        </Card>
    }
}
