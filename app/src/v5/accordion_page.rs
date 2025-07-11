use leptos::prelude::*;

use leptos_bootstrap::v5::{Accordion, AccordionItem, Card, CardBody, CardHeader};

const EXAMPLE: &str = "use leptos_bootstrap::v5::{Accordion, AccordionItem};

#[component]
pub fn Component() -> impl IntoView {
    view! {
        <Accordion>
            <AccordionItem header=\"Item #1\">
                {\"Content #1\"}
            </AccordionItem>
            <AccordionItem header=\"Item #2\">
                {\"Content #2\"}
            </AccordionItem>
        </Accordion>
    }
}";

#[component]
pub fn AccordionPage() -> impl IntoView {
    view! {
        <h1 class="mt-3 mb-3">Accordion</h1>
        <Card>
            <CardHeader>Accordion</CardHeader>
            <CardBody>
                <Accordion>
                    <AccordionItem header="Item #1">{"Content #1"}</AccordionItem>
                    <AccordionItem header="Item #2">{"Content #2"}</AccordionItem>
                </Accordion>
                <Card class="mt-2">
                    <CardBody>
                        <pre>{EXAMPLE}</pre>
                    </CardBody>
                </Card>
            </CardBody>
        </Card>
    }
}
