use material_yewi::button::{Button, ButtonColor, ButtonSize, ButtonVariant};
use yew::{html, Html};

pub fn render() -> Html {
    html! {
        <ButtonRow>
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
}

use yew::{classes, function_component, Children, Properties};

#[derive(Clone, PartialEq, Properties)]
struct WrapperProps {
    children: Children,
}

#[function_component(ButtonRow)]
fn button_row(props: &WrapperProps) -> Html {
    // A Small utility component for documentation, with some extra visual styling
    let wrapper_class = stylist::yew::use_style!(
        & > button { margin: 8px; }
    );
    html! {
        <div class={classes![wrapper_class]}>
            { for props.children.iter() }
        </div>
    }
}
