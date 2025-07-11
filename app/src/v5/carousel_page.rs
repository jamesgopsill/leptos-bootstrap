use leptos::prelude::*;

use leptos_bootstrap::v5::{Card, CardBody, CardHeader, Carousel, CarouselItem};

const EXAMPLE: &str = "use leptos_bootstrap::v5::{Card, CardHeader, CardBody};

#[component]
pub fn Component() -> impl IntoView {
view! {
<Carousel>
    <CarouselItem active=true>
        <img... />
    </CarouselItem>
    <CarouselItem>
        <img... />
    </CarouselItem>
</Carousel>
}";

#[component]
pub fn CarouselPage() -> impl IntoView {
    view! {
        <h1 class="mt-3 mb-3">Carousel</h1>
        <Card class="mb-3">
            <CardHeader>Carousel</CardHeader>
            <CardBody>
                <Carousel>
                    <CarouselItem active=true>
                        <svg
                            aria-label="Placeholder: First slide"
                            class="bd-placeholder-img bd-placeholder-img-lg d-block w-100"
                            height="400"
                            preserveAspectRatio="xMidYMid slice"
                            role="img"
                            width="800"
                            xmlns="http://www.w3.org/2000/svg"
                        >
                            <title>Placeholder</title>
                            <rect width="100%" height="100%" fill="#777"></rect>
                            <text x="50%" y="50%" fill="#555" dy=".3em">
                                {"First slide"}
                            </text>
                        </svg>
                    </CarouselItem>
                    <CarouselItem>
                        <svg
                            aria-label="Placeholder: First slide"
                            class="bd-placeholder-img bd-placeholder-img-lg d-block w-100"
                            height="400"
                            preserveAspectRatio="xMidYMid slice"
                            role="img"
                            width="800"
                            xmlns="http://www.w3.org/2000/svg"
                        >
                            <title>Placeholder</title>
                            <rect width="100%" height="100%" fill="#777"></rect>
                            <text x="50%" y="50%" fill="#555" dy=".3em">
                                {"Second slide"}
                            </text>
                        </svg>
                    </CarouselItem>
                </Carousel>
                <Card class="mt-2">
                    <CardBody>
                        <pre>{EXAMPLE}</pre>
                    </CardBody>
                </Card>
            </CardBody>
        </Card>
    }
}
