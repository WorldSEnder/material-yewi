use material_yewi::typography::{Typography, TypographyVariant};
use yew::{html, Html};

pub fn render() -> Html {
    html! {
        <TypographyListing>
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
}

use yew::{classes, function_component, Children, Properties};

#[derive(Clone, PartialEq, Properties)]
struct WrapperProps {
    children: Children,
}

#[function_component(TypographyListing)]
fn typography_listing(props: &WrapperProps) -> Html {
    // A small utility component for documentation, renders as div
    let wrapper_class = stylist::yew::use_style!(/* "typography-listing", */
        max-width: 500px;
    );
    html! {
        <div class={classes![wrapper_class]}>
            { for props.children.iter() }
        </div>
    }
}
