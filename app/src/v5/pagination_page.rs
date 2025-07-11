use leptos::prelude::*;

use leptos_bootstrap::v5::{Card, CardBody, CardHeader, PageItem, Pagination};

const EXAMPLE: &str = "use leptos_bootstrap::v5::{PageItem, Pagination};

#[component]
pub fn Component() -> impl IntoView {
view! {
    <Pagination>
        <PageItem>Previous</PageItem>
        <PageItem active=true>1</PageItem>
        <PageItem>2</PageItem>
        <PageItem>3</PageItem>
        <PageItem>Next</PageItem>
    </Pagination>
}";

#[component]
pub fn PaginationPage() -> impl IntoView {
    view! {
        <h1 class="mt-3 mb-3">Pagination</h1>
        <Card class="mb-3">
            <CardHeader>Pagination</CardHeader>
            <CardBody>
                <Pagination>
                    <PageItem>Previous</PageItem>
                    <PageItem active=true>1</PageItem>
                    <PageItem>2</PageItem>
                    <PageItem>3</PageItem>
                    <PageItem>Next</PageItem>
                </Pagination>
                <Card class="mt-2">
                    <CardBody>
                        <pre>{EXAMPLE}</pre>
                    </CardBody>
                </Card>
            </CardBody>
        </Card>
    }
}
