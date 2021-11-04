use material_yewi::button::Button;
use material_yewi::button_group::{ButtonGroup, Orientation};
use yew::{html, Html};

pub fn render() -> Html {
    html! {
        <ButtonGroup orientation={Orientation::Vertical}>
            <Button>
                {"One"}
            </Button>
            <Button>
                {"Two"}
            </Button>
            <Button>
                {"Three"}
            </Button>
        </ButtonGroup>
    }
}
