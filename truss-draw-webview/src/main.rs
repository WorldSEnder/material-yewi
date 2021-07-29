#![feature(generators, min_type_alias_impl_trait)]

use yew::prelude::*;
use material_yewi::button::Button;

/*
use yew_generator::{ContextLink, GenerativeComponent, GeneratorProvider, WrapGeneratorComponent};
pub struct Test;
impl GeneratorProvider for Test {
    type TProp = ();
    type Gen = impl GenerativeComponent;
    fn start(_: Self::TProp, _ctx: ContextLink) -> Self::Gen {
        move || {
            loop {
                yield yew::html! {
                    <div id="test-id" />
                }
            }
        }
    }
}
pub type TestComponent = WrapGeneratorComponent<Test>;
*/

enum Msg {
    AddOne,
}

struct Model {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    value: i64,
    typography: String,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let typography = css_in_rust::Style::create("typography", r#"
        & {
        }
        "#).expect("style compilation failure").to_string();
        Self { link, value: 0, typography }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let button_html = |i| html! {
            <Button>
                {format!("My Button #{:02}", i)}
            </Button>
        };
        html! {
            <>
                { for (0..100).map(button_html) }
            </>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
