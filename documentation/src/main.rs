use material_yewi::button::Button;
use material_yewi::typography::{Typography, TypographyVariant};
use material_yewi_documentation_macros::document_example;
use yew::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

struct Model {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    _link: ComponentLink<Self>,
    value: i64,
}

fn typography_example() -> Html {
    document_example! {r##"
    ::yew::html! {
        <>
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
        </>
    }
    "##}
}

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            _link: link,
            value: 0,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        {
            self.value += 1;
            true
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let on_button = Callback::from(|_| gloo::console::info!("button pressed"));

        html! {
            <>
                <Typography variant={TypographyVariant::H2}>
                    {"Buttons"}
                </Typography>
                <Button on_pressed={on_button}>
                    {"My Button"}
                </Button>
                <Typography variant={TypographyVariant::H2}>
                    {"Typography"}
                </Typography>
                <Typography variant={TypographyVariant::Paragraph}>
                    {"Below you can find examples showing the "}<pre style="display:inline;">{"Typography"}</pre>{" component."}
                </Typography>
                <div style={"width: 100%; max-width: 500px; margin-left: 10px;"}>
                    {typography_example()}
                </div>
            </>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
