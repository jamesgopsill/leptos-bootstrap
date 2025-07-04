use leptos::prelude::*;
use leptos_bootstrap::v5::{
    ContainerFluid, Icon, IconKind, NavBar, NavBarBrand, NavBarDropDown, NavBarDropDownItem,
    NavBarMenu, NavBarNav, NavBarText, NavLink,
};
use leptos_router::components::*;
use leptos_router::path;

use crate::v5::AccordionPage;
use crate::v5::InputPage;
use crate::v5::{ButtonPage, IndexPage, NotFoundPage};

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
                                <NavBarDropDownItem href="/leptos-bootstrap/v5/button">
                                    Button
                                </NavBarDropDownItem>
                                <NavBarDropDownItem href="/leptos-bootstrap/v5/input">
                                    Input
                                </NavBarDropDownItem>
                            </NavBarDropDown>
                        </NavBarNav>
                        <NavBarNav class="ms-auto">
                            <NavLink href="https://github.com/jamesgopsill/leptos-bootstrap" class="ms-auto"><Icon kind=IconKind::Github /></NavLink>
                        </NavBarNav>
                    </NavBarMenu>
                </ContainerFluid>
            </NavBar>
            <ContainerFluid>
                <Routes fallback=NotFoundPage>
                    <Route path=path!("/leptos-bootstrap/") view=IndexPage />
                    <Route path=path!("/leptos-bootstrap/v5/accordion") view=AccordionPage />
                    <Route path=path!("/leptos-bootstrap/v5/button") view=ButtonPage />
                    <Route path=path!("/leptos-bootstrap/v5/input") view=InputPage />
                </Routes>
            </ContainerFluid>
            <footer class="mt-5 mb-5">
                <p class="text-center">Ported with <Icon kind=IconKind::HeartFill class="ms-1 me-1" /> and <Icon kind=IconKind::CupHotFill class="ms-1 me-1" />.</p>
            </footer>
        </Router>
    }
}
