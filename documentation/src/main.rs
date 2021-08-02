use css_in_rust::bindings::yew::use_scopes;
use material_yewi::button::{Button, ButtonColor, ButtonSize, ButtonVariant};
use material_yewi::typography::{Typography, TypographyVariant};
use material_yewi_documentation_macros::document_example;
use std::convert::TryInto;
use yew::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn button_example() -> Html {
    #[derive(Clone, PartialEq, Properties)]
    struct WrapperProps {
        children: Children,
    }
    #[function_component(ButtonRow)]
    fn button_row(props: &WrapperProps) -> Html {
        let wrapper_class = use_scopes(
            "button-row",
            "& > button { margin: 8px; }"
                .to_string()
                .try_into()
                .unwrap(),
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

fn typography_example() -> Html {
    #[derive(Clone, PartialEq, Properties)]
    struct WrapperProps {
        children: Children,
    }
    #[function_component(TypographyListing)]
    fn typography_listing(props: &WrapperProps) -> Html {
        let wrapper_class = use_scopes(
            "typography-listing",
            "max-width: 500px;".to_string().try_into().unwrap(),
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

#[function_component(Docs)]
fn documentation() -> Html {
    let button_row = use_scopes(
        "button-row",
        r##"
        width: 100%;
        margin-left: 10px;
        "##
        .to_string()
        .try_into()
        .unwrap(),
    );
    let typography_row = use_scopes(
        "button-row",
        r##"
        width: 100%;
        margin-left: 10px;
        "##
        .to_string()
        .try_into()
        .unwrap(),
    );

    html! {
        <>
            <Typography variant={TypographyVariant::H2}>
                {"Buttons"}
            </Typography>
            <div class={classes![&button_row]}>
                {button_example()}
            </div>
            <Typography variant={TypographyVariant::H2}>
                {"Typography"}
            </Typography>
            <Typography variant={TypographyVariant::Paragraph}>
                {"Below you can find examples showing the "}<pre style="display:inline;">{"Typography"}</pre>{" component."}
            </Typography>
            <div class={classes![&typography_row]}>
                {typography_example()}
            </div>
        </>
    }
}

fn main() {
    yew::start_app::<Docs>();
}
