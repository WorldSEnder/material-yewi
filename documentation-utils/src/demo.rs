use material_styles_yew::use_theme;
use stylist::ast::sheet;
use stylist::manager::StyleManager;
use stylist::yew::use_style;
use yew::classes;
use yew::function_component;
use yew::html;
use yew::use_effect;
use yew::use_ref;
use yew::use_state;
use yew::Children;
use yew::ContextProvider;
use yew::NodeRef;
use yew::Properties;

#[derive(Default, Clone, PartialEq, Debug, Properties)]
pub struct DemoProperties {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Demo)]
pub fn demo(props: &DemoProperties) -> Html {
    type StyleProvider = ContextProvider<StyleManager>;
    let style_root_ref = use_ref(NodeRef::default);
    let manager = use_state(|| Option::None);
    {
        let style_root_ref = style_root_ref.clone();
        let manager = manager.clone();
        let _ = use_effect(move || {
            if !manager.is_some() {
                let node_ref = style_root_ref.borrow().clone();
                let node = node_ref.get().expect("Node to exist");
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
    let wrapper_style = use_style(/* "demo", */ sheet!(
        display: flex;
        margin: 0 10px;
    ));
    let frame_sheet = use_theme(|_theme| {
        let shadows2 = "0px 3px 1px -2px rgba(0,0,0,0.2),0px 2px 2px 0px rgba(0,0,0,0.14),0px 1px 5px 0px rgba(0,0,0,0.12)";
        sheet!(
            border: 0;
            flex-grow: 1;
            max-height: 400px;
            overflow-x: scroll;
            box-shadow: ${shadows2};
        )
    });
    let frame_style = use_style(/* "demo", */ frame_sheet.clone());

    let div_ref = style_root_ref.borrow().clone();
    let fake_iframe = "div";
    // FIXME: wrap into an iframe, inject a portal when the iframe loads
    html! {
        <div class={classes!["error-boundary", wrapper_style]} ref={div_ref}>
            <@{fake_iframe} class={classes![frame_style]}>
                <StyleProvider context={manager.as_ref().cloned().unwrap_or_default()}>
                    { for props.children.iter() }
                </StyleProvider>
            </@>
        </div>
    }
}
