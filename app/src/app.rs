use leptos::prelude::*;
use leptos_bootstrap::v5::{
    Container, ContainerFluid, Icon, IconKind, NavBar, NavBarBrand, NavBarDropDown,
    NavBarDropDownItem, NavBarMenu, NavBarNav, NavBarText, NavLink,
};
use leptos_router::components::*;
use leptos_router::path;

use crate::v5::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <NavBar class="navbar-expand-lg bg-body-tertiary">
                <ContainerFluid>
                    <NavBarBrand href="/leptos-bootstrap/">Leptos Bootstrap</NavBarBrand>
                    <NavBarMenu>
                        <NavBarNav>
                            <NavBarText class="me-2">{"Version: 5"}</NavBarText>
                            <NavBarDropDown label="Components">
                                <NavBarDropDownItem href="/leptos-bootstrap/v5/accordion">
                                    Accordion
                                </NavBarDropDownItem>
                                <NavBarDropDownItem href="/leptos-bootstrap/v5/alert">
                                    Alert
                                </NavBarDropDownItem>
                                <NavBarDropDownItem href="/leptos-bootstrap/v5/badge">
                                    Badge
                                </NavBarDropDownItem>
                                <NavBarDropDownItem href="/leptos-bootstrap/v5/breadcrumb">
                                    Breadcrumb
                                </NavBarDropDownItem>
                                <NavBarDropDownItem href="/leptos-bootstrap/v5/button">
                                    Button
                                </NavBarDropDownItem>
                                <NavBarDropDownItem href="/leptos-bootstrap/v5/card">
                                    Card
                                </NavBarDropDownItem>
                                <NavBarDropDownItem href="/leptos-bootstrap/v5/carousel">
                                    Carousel
                                </NavBarDropDownItem>
                                <NavBarDropDownItem href="/leptos-bootstrap/v5/dropdown">
                                    Dropdown
                                </NavBarDropDownItem>
                                <NavBarDropDownItem href="/leptos-bootstrap/v5/icon">
                                    Icon
                                </NavBarDropDownItem>
                                <NavBarDropDownItem href="/leptos-bootstrap/v5/input">
                                    Input
                                </NavBarDropDownItem>
                                <NavBarDropDownItem href="/leptos-bootstrap/v5/modal">
                                    Modal
                                </NavBarDropDownItem>
                                <NavBarDropDownItem href="/leptos-bootstrap/v5/pagination">
                                    Pagination
                                </NavBarDropDownItem>
                            </NavBarDropDown>
                        </NavBarNav>
                        <NavBarNav class="ms-auto">
                            <NavLink href="https://crates.io/crates/leptos-bootstrap">
                                crates.io
                            </NavLink>
                            <NavLink href="https://docs.rs/leptos-bootstrap/latest/leptos_bootstrap/">
                                docs.rs
                            </NavLink>
                            <NavLink href="https://github.com/jamesgopsill/leptos-bootstrap">
                                <Icon kind=IconKind::Github />
                            </NavLink>
                        </NavBarNav>
                    </NavBarMenu>
                </ContainerFluid>
            </NavBar>
            <Container>
                <Routes fallback=NotFoundPage>
                    <Route path=path!("/leptos-bootstrap/") view=IndexPage />
                    <Route path=path!("/leptos-bootstrap/v5/accordion") view=AccordionPage />
                    <Route path=path!("/leptos-bootstrap/v5/alert") view=AlertPage />
                    <Route path=path!("/leptos-bootstrap/v5/badge") view=BadgePage />
                    <Route path=path!("/leptos-bootstrap/v5/breadcrumb") view=BreadcrumbPage />
                    <Route path=path!("/leptos-bootstrap/v5/button") view=ButtonPage />
                    <Route path=path!("/leptos-bootstrap/v5/card") view=CardPage />
                    <Route path=path!("/leptos-bootstrap/v5/carousel") view=CarouselPage />
                    <Route path=path!("/leptos-bootstrap/v5/dropdown") view=DropDownPage />
                    <Route path=path!("/leptos-bootstrap/v5/icon") view=IconPage />
                    <Route path=path!("/leptos-bootstrap/v5/input") view=InputPage />
                    <Route path=path!("/leptos-bootstrap/v5/modal") view=ModalPage />
                    <Route path=path!("/leptos-bootstrap/v5/pagination") view=PaginationPage />
                </Routes>
            </Container>
            <footer class="mt-5 mb-5">
                <p class="text-center">
                    Ported with <Icon kind=IconKind::HeartFill class="ms-1 me-1" />and
                    <Icon kind=IconKind::CupHotFill class="ms-1 me-1" />.
                </p>
            </footer>
        </Router>
    }
}
