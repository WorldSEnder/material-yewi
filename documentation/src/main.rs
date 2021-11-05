use material_yewi::app_bar::{AppBar, AppBarColor, AppBarPosition};
use material_yewi::link::Link;
use material_yewi::toolbar::Toolbar;
use material_yewi::typography::{Typography, TypographyVariant};
use yew::prelude::*;
use yew_router::{Routable, Router};

mod app_bar;
mod button;
mod button_group;
mod paper;
mod typography;

#[derive(PartialEq, Clone, Routable)]
pub enum DocRoute {
    #[at("/")]
    Home,
    #[at("/appbar")]
    AppBar,
    #[at("/buttons")]
    Buttons,
    #[at("/button-group")]
    ButtonGroup,
    #[at("/paper")]
    Paper,
    #[at("/typography")]
    Typography,
    #[not_found]
    #[at("/404")]
    NotFound,
}

type DocRouter = Router<DocRoute>;
type DocLink = Link<DocRoute>;

#[function_component(Home)]
fn home() -> Html {
    html! {
        <>
            <DocLink route={DocRoute::AppBar}>{"App Bar"}</DocLink>
            <DocLink route={DocRoute::Buttons}>{"Buttons"}</DocLink>
            <DocLink route={DocRoute::ButtonGroup}>{"Button Group"}</DocLink>
            <DocLink route={DocRoute::Paper}>{"Paper"}</DocLink>
            <DocLink route={DocRoute::Typography}>{"Typography"}</DocLink>
        </>
    }
}

#[function_component(NotFound)]
fn not_found() -> Html {
    html! {
        <>
            <Typography variant={TypographyVariant::Body2}>{"The page you were searching for could not be found."}</Typography>
            <DocLink route={DocRoute::Home}>{"Back to home page"}</DocLink>
        </>
    }
}

#[function_component(Docs)]
fn documentation() -> Html {
    let page_header = || {
        let title = "Material Yewi";
        let caption = "Beautifully styled components in Yew";
        html! {
            <>
                <AppBar position={AppBarPosition::Static} color={AppBarColor::Transparent}><Toolbar>
                    <DocLink route={DocRoute::Home} variant={TypographyVariant::H1}>{title}</DocLink>
                    <Typography variant={TypographyVariant::Caption}>{caption}</Typography>
                </Toolbar></AppBar>
            </>
        }
    };

    fn switch(route: &DocRoute) -> Html {
        match route {
            DocRoute::Home => html! { <Home /> },
            DocRoute::AppBar => html! { <app_bar::Doc /> },
            DocRoute::Buttons => html! { <button::Doc /> },
            DocRoute::ButtonGroup => html! { <button_group::Doc /> },
            DocRoute::Paper => html! { <paper::Doc /> },
            DocRoute::Typography => html! { <typography::Doc /> },
            DocRoute::NotFound => html! { <NotFound /> },
        }
    }
    html! {
        <>
            <stylist::yew::Global css={stylist::css!(
                * {
                    padding: 0;
                    margin: 0;
                }
                body {
                    font-family: Roboto, serif;
                }
                // See https://stackoverflow.com/questions/4192277/
                // Disables scroll-x on mobile devices. Might regret this later
                html, body {
                    overflow-x: hidden;
                }
                body {
                    position: relative;
                }
            )} />
            {page_header()}
            <DocRouter render={Router::render(switch)} />
        </>
    }
}

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn main() {
    yew::start_app::<Docs>();
}
