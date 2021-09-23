use material_yewi::button::{Button, ButtonColor, ButtonSize, ButtonVariant};
use material_yewi::typography::{Typography, TypographyVariant};
use material_yewi_documentation_macros::document_example;
use stylist::ast::sheet;
use stylist::yew::use_style;
use yew::prelude::*;

fn example() -> Html {
    #[derive(Clone, PartialEq, Properties)]
    struct WrapperProps {
        children: Children,
    }
    #[function_component(ButtonRow)]
    fn button_row(props: &WrapperProps) -> Html {
        let wrapper_class = use_style(/* "button-row", */ sheet!(
            & > button { margin: 8px; }
        ));
        ::yew::html! {
            <div class={classes![wrapper_class]}>
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

#[function_component(Doc)]
pub fn doc() -> Html {
    html! {
        <>
            <Typography variant={TypographyVariant::H2}>
                {"Buttons"}
            </Typography>
            {example()}
        </>
    }
}
