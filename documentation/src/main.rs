use material_yewi::typography::{Typography, TypographyVariant};
use yew::prelude::*;
use yew_router::{components::Link, Routable, Router};

mod button_page;
mod typography_page;

#[derive(PartialEq, Clone, Routable)]
pub enum DocRoute {
    #[at("/")]
    Home,
    #[at("/buttons")]
    Buttons,
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
            <Typography variant={TypographyVariant::Button}>
                <DocLink route={DocRoute::Buttons}>{"Buttons"}</DocLink>
            </Typography>
            <Typography variant={TypographyVariant::Button}>
                <DocLink route={DocRoute::Typography}>{"Typography"}</DocLink>
            </Typography>
        </>
    }
}

#[function_component(NotFound)]
fn not_found() -> Html {
    html! {
        <>
            <Typography variant={TypographyVariant::Body2}>{"The page you were searching for could not be found."}</Typography>
            <Typography variant={TypographyVariant::Button}>
                <DocLink route={DocRoute::Home}>{"Back to home page"}</DocLink>
            </Typography>
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
                <Typography variant={TypographyVariant::H1}><DocLink route={DocRoute::Home}>{title}</DocLink></Typography>
                <Typography variant={TypographyVariant::Caption}>{caption}</Typography>
            </>
        }
    };

    fn switch(route: &DocRoute) -> Html {
        match route {
            DocRoute::Home => html! { <Home /> },
            DocRoute::Buttons => html! { <button_page::Doc /> },
            DocRoute::Typography => html! { <typography_page::Doc /> },
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
