#![feature(generators, generator_trait, never_type, min_type_alias_impl_trait)]

use yew::html::AnyScope;
use pin_cell::PinMut;
use pin_cell::PinCell;
use std::rc::Rc;
use std::pin::Pin;
use yew::Html;
use yew::Component;
use yew::ComponentLink;
use yew::Properties;
use std::ops::{Generator, GeneratorState};

/// A trait-alias for `Generator<(), Return=!, Yield=Html>`, this is what
/// a generator-based component is supposed to implement as state.
pub trait GenerativeComponent: Generator<(), Return=!, Yield=Html> {}
impl<G: Generator<(), Return=!, Yield=Html>> GenerativeComponent for G {}

/// Much like [`ComponentLink`], connects back to the environment around
/// the component.
pub struct ContextLink {
    send_message: Box<dyn Fn(GeneratorMessage)>,
    scope: AnyScope,
}

/// Implement this trait on a dummy struct that is passed as argument to
/// [`WrapGeneratorComponent`].
pub trait GeneratorProvider {
    type TProp: Properties + PartialEq;
    type Gen: Generator<(), Return=!, Yield=Html>;

    fn start(props: Self::TProp, ctx: ContextLink) -> Self::Gen;
}

pub enum GeneratorMessage {
    Poke,
    Callback(Box<dyn FnOnce() -> bool>),
}

/// Wrapper implement [`Component`] for the Generator.
pub struct WrapGeneratorComponent<G: 'static + GeneratorProvider> {
    props: G::TProp,
    state: Pin<Rc<PinCell<G::Gen>>>,
    self_link: ComponentLink<Self>,
}

fn with_context<G, F, R>(s: &ComponentLink<WrapGeneratorComponent<G>>, f: F) -> R
where F: FnOnce(ContextLink) -> R,
    G: 'static + GeneratorProvider
{
    let s = s.clone();
    let scope = s.clone().into();
    let ctx = ContextLink {
        send_message: Box::new(move |m| s.send_message(m)),
        scope,
    };
    f(ctx)
}

impl<G: 'static +  GeneratorProvider> Component for WrapGeneratorComponent<G> {
    type Message = GeneratorMessage;
    type Properties = G::TProp;

    fn create(props: Self::Properties, self_link: ComponentLink<Self>) -> Self {
        let saved_props = props.clone();
        let gen = with_context(&self_link, |ctx| G::start(props, ctx));
        WrapGeneratorComponent {
            state: Rc::pin(PinCell::new(gen)),
            props: saved_props,
            self_link,
        }
    }

    fn view(&self) -> Html {
        let mut internal_ref = self.state.as_ref().borrow_mut();
        let state_ref = PinMut::as_mut(&mut internal_ref);

        match state_ref.resume(()) {
            GeneratorState::Yielded(y) => y,
            GeneratorState::Complete(nope) => match nope {},
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            GeneratorMessage::Poke => true,
            GeneratorMessage::Callback(cb) => cb(),
        }
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        let mut props = props;
        std::mem::swap(&mut self.props, &mut props);
        let changed = props != self.props;

        if changed {
            let gen = with_context(&self.self_link, |ctx| G::start(self.props.clone(), ctx));
            self.state = Rc::pin(PinCell::new(gen));
        }

        changed
    }

    fn rendered(&mut self, _first_render: bool) {}

    fn destroy(&mut self) {}
}

impl ContextLink {
    /// Poke the context to demand an update to the underlying component. This will
    /// trigger `Component::view`, advancing the generator.
    pub fn poke(&self) {
        (*self.send_message)(GeneratorMessage::Poke);
    }

    /// Get the scope of the underlying component.
    pub fn scope(&self) -> &AnyScope {
        &self.scope
    }
}

#[cfg(tests)]
pub mod tests;
