use super::*;

pub struct Test;
impl GeneratorProvider for Test {
    type TProp = ();
    type Gen = impl GenerativeComponent;
    fn start(_: (), _ctx: ContextLink) -> Self::Gen {
        move || {
            loop {
                // ctx.poke();
                yield yew::html! {
                    <div id="test-id" />
                }
            }
        }
    }
}
pub type TestComponent = WrapGeneratorComponent<Test>;
