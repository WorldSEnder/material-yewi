use material_styles_yew::use_theme;
use stylist::ast::sheet;
use stylist::manager::StyleManager;
use stylist::yew::use_style;
use yew::classes;
use yew::function_component;
use yew::html;
use yew::use_effect;
use yew::use_node_ref;
use yew::use_state;
use yew::Children;
use yew::ContextProvider;
use yew::Html;
use yew::NodeRef;
use yew::Properties;

use crate::shadow_div::ShadowDiv;

#[derive(Default, Clone, PartialEq, Debug, Properties)]
struct DemoWrapperProperties {
    pub children: Children,
}

#[function_component]
fn DemoWrapper(props: &DemoWrapperProperties) -> Html {
    let frame_sheet = use_theme(|theme| {
        sheet!(
            border: 0;
            flex-grow: 1;
            max-height: 400px;
            overflow-y: auto;
            box-shadow: ${&theme.shadows[2]};
            padding: 24px;
            margin: auto;
            display: flex;
            justify-content: safe center;
            background-color: #F0F2F3;
        )
    });
    let frame_style = use_style(/* "demo", */ frame_sheet.clone());

    html! {
        <div class={classes![frame_style]}>
            { for props.children.iter() }
        </div>
    }
}

#[derive(Default, Clone, PartialEq, Debug, Properties)]
struct DemoContextProperties {
    pub demo_host_ref: NodeRef,
    pub children: Children,
}

#[function_component]
fn DemoContextWrapper(props: &DemoContextProperties) -> Html {
    type StyleProvider = ContextProvider<StyleManager>;
    let manager = use_state(|| Option::None);
    {
        let demo_host_ref = props.demo_host_ref.clone();
        let manager = manager.clone();
        let _ = use_effect(move || {
            if !manager.is_some() {
                let node = demo_host_ref.get().expect("Node to exist");
                let mgr = StyleManager::builder()
                    .container(node)
                    .prefix("demo".into())
                    .build()
                    .expect("Manager to build");
                manager.set(Some(mgr));
            }
            || {}
        });
    }

    html! {
        <StyleProvider context={manager.as_ref().cloned().unwrap_or_default()}>
            <DemoWrapper>
                { for props.children.iter() }
            </DemoWrapper>
        </StyleProvider>
    }
}

#[derive(Default, Clone, PartialEq, Debug, Properties)]
pub struct DemoProperties {
    #[prop_or_default]
    pub children: Children,
}

#[function_component]
pub fn Demo(props: &DemoProperties) -> Html {
    let style_root_ref = use_node_ref();
    let wrapper_style = use_style!(/* "demo", */
        display: flex;
        margin: 0 10px;
        padding-bottom: 20px;
    );

    html! {
        <ShadowDiv class={classes!["error-boundary", wrapper_style]} inner_ref={&style_root_ref}>
            <DemoContextWrapper demo_host_ref={&style_root_ref}>
                { for props.children.iter() }
            </DemoContextWrapper>
        </ShadowDiv>
    }
}
