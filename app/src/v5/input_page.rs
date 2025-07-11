use leptos::prelude::*;

use leptos_bootstrap::v5::{Card, CardBody, CardHeader, FloatingLabel, Input, InputKind};

const EXAMPLE_ONE: &str = "use leptos_bootstrap::v5::{Input};

#[component]
pub fn Component() -> impl IntoView {
    let text = RwSignal::new(String::new());
    view! {
        <Input value=text placeholder=\"Text\" />
    }
}";

const EXAMPLE_TWO: &str = "use leptos_bootstrap::v5::{Input, InputKind};

#[component]
pub fn Component() -> impl IntoView {
    let email = RwSignal::new(String::new());
    view! {
        <Input kind=InputKind::Email value=email placeholder=\"Email\" />
    }
}";

const EXAMPLE_THREE: &str = "use leptos_bootstrap::v5::{Input};

#[component]
pub fn Component() -> impl IntoView {
    let datetime_local = RwSignal::new(String::new());
    view! {
        <Input kind=InputKind::DateTimeLocal />
    }
}";

const EXAMPLE_FOUR: &str = "use leptos_bootstrap::v5::{TextInput};

#[component]
pub fn Component() -> impl IntoView {
    let val = RwSignal::new(String::new());
    view! {
        <FloatingLabel label=\"Email\">
            <EmailInput value=val />
        </FloatingLabel>
    }
}";

#[component]
pub fn InputPage() -> impl IntoView {
    let text = RwSignal::new(String::new());
    let email = RwSignal::new(String::new());
    let datetime_local = RwSignal::new(String::new());
    let floating = RwSignal::new(String::new());

    view! {
        <h1 class="mt-3 mb-3">Inputs</h1>
        <Card>
            <CardHeader>Text</CardHeader>
            <CardBody>
                <Input value=text placeholder="Text" />
                <Card class="mt-2">
                    <CardBody>
                        <pre>{EXAMPLE_ONE}</pre>
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
                        <pre>{EXAMPLE_TWO}</pre>
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
                        <pre>{EXAMPLE_THREE}</pre>
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
                        <pre>{EXAMPLE_FOUR}</pre>
                    </CardBody>
                </Card>
            </CardBody>
        </Card>
    }
}
