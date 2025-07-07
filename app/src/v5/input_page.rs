use leptos::prelude::*;

use leptos_bootstrap::v5::{Card, CardBody, CardHeader, FloatingLabel, Input, InputKind};

#[component]
pub fn InputPage() -> impl IntoView {
    let text = RwSignal::new(String::new());
    let email = RwSignal::new(String::new());
    let datetime_local = RwSignal::new(String::new());
    let floating = RwSignal::new(String::new());

    view! {
        <h1>Inputs</h1>
        <Card>
            <CardHeader>Text</CardHeader>
            <CardBody>
                <Input value=text placeholder="Text" />
                <Card class="mt-2">
                    <CardBody>
                        <pre>
"use leptos_bootstrap::v5::{Input};

#[component]
pub fn Component() -> impl IntoView {
    let text = RwSignal::new(String::new());
    view! {
        <Input value=text placeholder=\"Text\" />
    }
}"
                        </pre>
                    </CardBody>
                </Card>
            </CardBody>
        </Card>
        <Card class="mt-3">
            <CardHeader>Email</CardHeader>
            <CardBody>
                <Input kind=InputKind::Email value=email placeholder="Email" />
                <Card class="mt-2">
                    <CardBody>
                        <pre>
"use leptos_bootstrap::v5::{Input, InputKind};

#[component]
pub fn Component() -> impl IntoView {
    let email = RwSignal::new(String::new());
    view! {
        <Input kind=InputKind::Email value=email placeholder=\"Email\" />
    }
}"
                        </pre>
                    </CardBody>
                </Card>
            </CardBody>
        </Card>
        <Card class="mt-3">
            <CardHeader>{"Datetime-Local"}</CardHeader>
            <CardBody>
                <Input kind=InputKind::DateTimeLocal value=datetime_local />
                <Card class="mt-2">
                    <CardBody>
                        <pre>
"use leptos_bootstrap::v5::{Input};

#[component]
pub fn Component() -> impl IntoView {
    let datetime_local = RwSignal::new(String::new());
    view! {
        <Input kind=InputKind::DateTimeLocal />
    }
}"
                        </pre>
                    </CardBody>
                </Card>
            </CardBody>
        </Card>
        <Card class="mt-3">
            <CardHeader>Floating Label</CardHeader>
            <CardBody>
                <FloatingLabel label="Email">
                    <Input kind=InputKind::Email value=floating />
                </FloatingLabel>
                <Card class="mt-2">
                    <CardBody>
                        <pre>
"use leptos_bootstrap::v5::{TextInput};

#[component]
pub fn Component() -> impl IntoView {
    let val = RwSignal::new(String::new());
    view! {
        <FloatingLabel label=\"Email\">
            <EmailInput value=val />
        </FloatingLabel>
    }
}"
                        </pre>
                    </CardBody>
                </Card>
            </CardBody>
        </Card>
    }
}
