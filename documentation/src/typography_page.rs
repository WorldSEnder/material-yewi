use material_yewi::typography::{Typography, TypographyVariant};
use material_yewi_documentation_macros::document_example;
use stylist::ast::sheet;
use stylist::yew::use_sheet;
use yew::prelude::*;

fn example() -> Html {
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

#[function_component(Doc)]
pub fn doc() -> Html {
    html! {
        <>
            <Typography variant={TypographyVariant::H2}>
                {"Typography"}
            </Typography>
            <Typography variant={TypographyVariant::Paragraph}>
                {"Below you can find examples showing the "}<pre style="display:inline;">{"Typography"}</pre>{" component."}
            </Typography>
            {example()}
        </>
    }
}
