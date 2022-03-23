use wasm_bindgen::JsCast;
use web_sys::{Element, ShadowRootInit, ShadowRootMode};
use yew::{create_portal, html, Children, Classes, Component, Context, Html, NodeRef, Properties};

#[derive(Properties, PartialEq)]
pub struct ShadowDivProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub inner_ref: NodeRef,
    #[prop_or_default]
    pub children: Children,
}

pub struct ShadowDiv {
    host_ref: NodeRef,
    inner_host: Option<Element>,
}

impl Component for ShadowDiv {
    type Message = ();
    type Properties = ShadowDivProps;

    fn create(_: &Context<Self>) -> Self {
        Self {
            host_ref: NodeRef::default(),
            inner_host: None,
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let shadow_root = self
                .host_ref
                .get()
                .expect("rendered host")
                .unchecked_into::<Element>()
                .attach_shadow(&ShadowRootInit::new(ShadowRootMode::Open))
                .expect("installing shadow root succeeds");
            let inner_host = gloo_utils::document()
                .create_element("div")
                .expect("can create inner wrapper");
            inner_host
                .set_attribute("style", "display: contents;")
                .expect("can set inner attribute");
            shadow_root
                .append_child(&inner_host)
                .expect("can attach inner host");
            self.inner_host = Some(inner_host);
            ctx.link().send_message(());
        }
    }

    fn update(&mut self, _: &Context<Self>, _: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let contents = if let Some(ref inner_host) = self.inner_host {
            create_portal(
                html! {
                    <div style={"display: contents;"} ref={props.inner_ref.clone()}>
                        {for props.children.iter()}
                    </div>
                },
                inner_host.clone(),
            )
        } else {
            html! { <></> }
        };
        html! {
            <div class={props.class.clone()} ref={self.host_ref.clone()}>
                {contents}
            </div>
        }
    }
}
