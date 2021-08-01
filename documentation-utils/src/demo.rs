use yew::function_component;
use yew::html;
use yew::Children;
use yew::Properties;

#[derive(Default, Clone, PartialEq, Debug, Properties)]
pub struct DemoProperties {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Demo)]
pub fn demo(props: &DemoProperties) -> Html {
    // FIXME: wrap into an iframe, inject a portal when the iframe loads
    html! {
        <div class="error-boundary">
            { for props.children.iter() }
        </div>
    }
}
