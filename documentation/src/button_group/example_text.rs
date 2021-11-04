use material_yewi::button::{Button, ButtonColor};
use material_yewi::button_group::ButtonGroup;
use yew::{html, Html};

pub fn render() -> Html {
    html! {
        <ButtonGroup>
            <Button>
                {"My Button"}
            </Button>
            <Button color={ButtonColor::Secondary}>
                {"Secondary"}
            </Button>
            <Button disabled={true}>
                {"Disabled"}
            </Button>
        </ButtonGroup>
    }
}
