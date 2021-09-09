use material_yewi::button::{Button, ButtonColor, ButtonSize, ButtonVariant};
use material_yewi::typography::{Typography, TypographyVariant};
use material_yewi_documentation_macros::document_example;
use stylist::ast::sheet;
use stylist::yew::use_sheet;
use yew::prelude::*;
use yew_router::{components::Link, Routable, Router};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(PartialEq, Clone, Routable)]
enum DocRoute {
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

fn button_example() -> Html {
    #[derive(Clone, PartialEq, Properties)]
    struct WrapperProps {
        children: Children,
    }
    #[function_component(ButtonRow)]
    fn button_row(props: &WrapperProps) -> Html {
        let wrapper_class = use_sheet(
            "button-row",
            sheet!(
                & > button { margin: 8px; }
            ),
        );
        ::yew::html! {
            <div class={classes![&wrapper_class]}>
                { for props.children.iter() }
            </div>
        }
    }

    document_example! {r##"
    ::yew::html! {
        <ButtonRow> // Small utility component for documentation, renders as div
            <Button>
                {"My Button"}
            </Button>
            <Button variant={ButtonVariant::Outlined}>
                {"Outlined"}
            </Button>
            <Button variant={ButtonVariant::Contained}>
                {"Contained"}
            </Button>
            <Button size={ButtonSize::Small}>
                {"Small"}
            </Button>
            <Button size={ButtonSize::Large}>
                {"Large"}
            </Button>
            <Button color={ButtonColor::Secondary}>
                {"Secondary"}
            </Button>
            <Button disabled={true}>
                {"Disabled"}
            </Button>
        </ButtonRow>
    }
    "##}
}

#[function_component(ButtonDoc)]
fn button_doc() -> Html {
    html! {
        <>
            <Typography variant={TypographyVariant::H2}>
                {"Buttons"}
            </Typography>
            {button_example()}
        </>
    }
}

fn typography_example() -> Html {
    #[derive(Clone, PartialEq, Properties)]
    struct WrapperProps {
        children: Children,
    }
    #[function_component(TypographyListing)]
    fn typography_listing(props: &WrapperProps) -> Html {
        let wrapper_class = use_sheet(
            "typography-listing",
            sheet!(
                max-width: 500px;
            ),
        );
        ::yew::html! {
            <div class={classes![&wrapper_class]}>
                { for props.children.iter() }
            </div>
        }
    }

    document_example! {r##"
    ::yew::html! {
        <TypographyListing> // Small utility component for documentation, renders as div
            <Typography variant={TypographyVariant::H1} gutter_bottom={true}>
                {"h1. Heading"}
            </Typography>
            <Typography variant={TypographyVariant::H2} gutter_bottom={true}>
                {"h2. Heading"}
            </Typography>
            <Typography variant={TypographyVariant::H3} gutter_bottom={true}>
                {"h3. Heading"}
            </Typography>
            <Typography variant={TypographyVariant::H4} gutter_bottom={true}>
                {"h4. Heading"}
            </Typography>
            <Typography variant={TypographyVariant::H5} gutter_bottom={true}>
                {"h5. Heading"}
            </Typography>
            <Typography variant={TypographyVariant::H6} gutter_bottom={true}>
                {"h6. Heading"}
            </Typography>
            <Typography variant={TypographyVariant::Subtitle1} gutter_bottom={true}>
                {"subtitle1. Lorem ipsum dolor sit amet, consectetur adipisicing elit. Quos
                blanditiis tenetur"}
            </Typography>
            <Typography variant={TypographyVariant::Subtitle2} gutter_bottom={true}>
                {"subtitle2. Lorem ipsum dolor sit amet, consectetur adipisicing elit. Quos
                blanditiis tenetur"}
            </Typography>
            <Typography variant={TypographyVariant::Body1} gutter_bottom={true}>
                {"body1. Lorem ipsum dolor sit amet, consectetur adipisicing elit. Quos
                blanditiis tenetur unde suscipit, quam beatae rerum inventore consectetur,
                neque doloribus, cupiditate numquam dignissimos laborum fugiat deleniti? Eum
                quasi quidem quibusdam."}
            </Typography>
            <Typography variant={TypographyVariant::Body2} gutter_bottom={true}>
                {"body2. Lorem ipsum dolor sit amet, consectetur adipisicing elit. Quos
                blanditiis tenetur unde suscipit, quam beatae rerum inventore consectetur,
                neque doloribus, cupiditate numquam dignissimos laborum fugiat deleniti? Eum
                quasi quidem quibusdam."}
            </Typography>
            <Typography variant={TypographyVariant::Button} gutter_bottom={true}>
                {"button text"}
            </Typography>
            <Typography variant={TypographyVariant::Caption} gutter_bottom={true}>
                {"caption text"}
            </Typography>
            <Typography variant={TypographyVariant::Overline} gutter_bottom={true}>
                {"overline text"}
            </Typography>
        </TypographyListing>
    }
    "##}
}

#[function_component(TypographyDoc)]
fn typography_doc() -> Html {
    html! {
        <>
            <Typography variant={TypographyVariant::H2}>
                {"Typography"}
            </Typography>
            <Typography variant={TypographyVariant::Paragraph}>
                {"Below you can find examples showing the "}<pre style="display:inline;">{"Typography"}</pre>{" component."}
            </Typography>
            {typography_example()}
        </>
    }
}

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
                <Typography variant={TypographyVariant::H1}>{title}</Typography>
                <Typography variant={TypographyVariant::Caption}>{caption}</Typography>
            </>
        }
    };

    fn switch(route: &DocRoute) -> Html {
        match route {
            DocRoute::Home => html! { <Home /> },
            DocRoute::Buttons => html! { <ButtonDoc /> },
            DocRoute::Typography => html! { <TypographyDoc /> },
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

fn main() {
    yew::start_app::<Docs>();
}
