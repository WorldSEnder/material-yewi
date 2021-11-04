use material_yewi::button::{Button, ButtonSize, ButtonVariant};
use material_yewi::button_group::ButtonGroup;
use yew::{html, Html};

pub fn render() -> Html {
    html! {
        <ButtonGroupAssortment>
            <ButtonGroup>
                <Button variant={ButtonVariant::Contained}>
                    {"One"}
                </Button>
                <Button variant={ButtonVariant::Contained}>
                    {"Two"}
                </Button>
                <Button variant={ButtonVariant::Contained}>
                    {"Three"}
                </Button>
            </ButtonGroup>
            <ButtonGroup>
                <Button variant={ButtonVariant::Outlined}>
                    {"One"}
                </Button>
                <Button variant={ButtonVariant::Outlined}>
                    {"Two"}
                </Button>
                <Button variant={ButtonVariant::Outlined}>
                    {"Three"}
                </Button>
            </ButtonGroup>
            <ButtonGroup>
                <Button variant={ButtonVariant::Text} size={ButtonSize::Small}>
                    {"One"}
                </Button>
                <Button variant={ButtonVariant::Text} size={ButtonSize::Small}>
                    {"Two"}
                </Button>
                <Button variant={ButtonVariant::Text} size={ButtonSize::Small}>
                    {"Three"}
                </Button>
            </ButtonGroup>
        </ButtonGroupAssortment>
    }
}

use yew::{classes, function_component, Children, Properties};

#[derive(Clone, PartialEq, Properties)]
struct WrapperProps {
    children: Children,
}

#[function_component(ButtonGroupAssortment)]
fn assortment(props: &WrapperProps) -> Html {
    // A Small utility component for documentation, with some extra visual styling
    let wrapper_class = stylist::yew::use_style!(
        & > * { margin: 8px; }
        display: flex;
        flex-direction: column;
        align-items: center;
    );
    html! {
        <div class={classes![wrapper_class]}>
            { for props.children.iter() }
        </div>
    }
}
