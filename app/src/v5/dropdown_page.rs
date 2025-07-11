use leptos::prelude::*;

use leptos_bootstrap::v5::{Card, CardBody, CardHeader, DropDown, DropDownItem};

const EXAMPLE: &str = "use leptos_bootstrap::v5::{DropDown, DropDownItem};

#[component]
pub fn Component() -> impl IntoView {
view! {
    <DropDown label=\"Dropdown\">
        <DropDownItem>{\"Item #1\"}</DropDownItem>
        <DropDownItem>{\"Item #2\"}</DropDownItem>
    </DropDown>
}";

#[component]
pub fn DropDownPage() -> impl IntoView {
    view! {
        <h1 class="mt-3 mb-3">Dropdown</h1>
        <Card class="mb-3">
            <CardHeader>Dropdown</CardHeader>
            <CardBody>
                <DropDown label="Dropdown">
                    <DropDownItem>{"Item #1"}</DropDownItem>
                    <DropDownItem>{"Item #2"}</DropDownItem>
                </DropDown>
                <Card class="mt-2">
                    <CardBody>
                        <pre>{EXAMPLE}</pre>
                    </CardBody>
                </Card>
            </CardBody>
        </Card>
    }
}
