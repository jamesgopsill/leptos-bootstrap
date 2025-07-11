use leptos::prelude::*;

use leptos_bootstrap::v5::{
    Card, CardBody, CardHeader, Modal, ModalBody, ModalFooter, ModalHeader,
};

const EXAMPLE: &str = "use leptos_bootstrap::v5::{Modal, ModalBody, ModalFooter, ModalHeader};

#[component]
pub fn Component() -> impl IntoView {
view! {
    <Modal class=\"position-static d-block\">
        <ModalHeader>{\"Modal Title\"}</ModalHeader>
        <ModalBody>{\"Modal Body\"}</ModalBody>
        <ModalFooter>{\"Modal Footer\"}</ModalFooter>
    </Modal>
";

#[component]
pub fn ModalPage() -> impl IntoView {
    view! {
        <h1 class="mt-3 mb-3">Modal</h1>
        <Card class="mb-3">
            <CardHeader>Modal</CardHeader>
            <CardBody>
                <Modal class="position-static d-block">
                    <ModalHeader>{"Modal Title"}</ModalHeader>
                    <ModalBody>{"Modal Body"}</ModalBody>
                    <ModalFooter>{"Modal Footer"}</ModalFooter>
                </Modal>
                <Card class="mt-2">
                    <CardBody>
                        <pre>{EXAMPLE}</pre>
                    </CardBody>
                </Card>
            </CardBody>
        </Card>
    }
}
