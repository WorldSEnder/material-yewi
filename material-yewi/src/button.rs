use yew::Children;
use css_in_rust::Style;
use yew::use_context;
use yew::Properties;
use yew::function_component;
use yew::html;
use yew::classes;
use css_in_rust::style::ast::Scopes;
use yew_styles::Theme;
use std::convert::TryInto;

#[derive(Debug, Clone, PartialEq)]
pub struct ButtonStyleRoot {
    css_scopes: Scopes,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ButtonStyleMain {
    css_scopes: Scopes,
}

#[derive(Default, Clone, PartialEq, Debug, Properties)]
pub struct ButtonProperties {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Button)]
pub fn button(props: &ButtonProperties) -> Html {
    let theme = use_context::<Theme>();
    let root_override = theme
        .as_ref()
        .and_then(|theme| theme.components.search_override::<ButtonStyleRoot>())
        .map(|c| &c.css_scopes);
    let _main_override = theme
        .as_ref()
        .and_then(|theme| theme.components.search_override::<ButtonStyleMain>())
        .map(|c| &c.css_scopes);

    let mut root_inline: Scopes =
        r#"
        & {
            display: inline-flex;
            align-items: center;
            justify-content: center;
            position: relative;
            box-sizing: border-box;
            background-color: transparent;
            outline: 0;
            border: 0;
            margin: 0;
            border-radius: 0;
            padding: 0;
            cursor: pointer;
            user-select: none;
            vertical-align: middle;
            text-decoration: none;
            color: inherit;

            -webkit-tap-highlight-color:transparent;
            -moz-appearance:none;
            -webkit-appearance:none;
        }
        &::-moz-focus-inner {
            border-style: none;
        }
        "#.to_string().try_into().expect("error in css parsing");
    let typograph_scopes =
        r#"
        & {
            font-family:"Roboto","Helvetica","Arial",sans-serif;
            font-weight:500;
            font-size:0.875rem;
            line-height:1.75;
            letter-spacing:0.02857em;
            text-transform:uppercase;
        }
        "#.to_string().try_into().expect("error in css parsing");
    let sizing_scopes =
        // TODO: compute from theme!
        r#"
        & {
            min-width: 64px;
            padding: 6px 8px;
            border-radius: 4px;
        }
        "#.to_string().try_into().expect("error in css parsing");
    let hover_scopes =
        r#"
        &:hover {
            text-decoration: none;
            background-color: rgba(25, 118, 210, 0.04);
        }
        "#.to_string().try_into().expect("error in css parsing");
    root_inline.append(typograph_scopes);
    root_inline.append(sizing_scopes);
    root_inline.append(hover_scopes);
    root_inline.append(root_override.cloned().unwrap_or_default());

    let root_style = Style::from_scopes(
        "Mwi-button-root",
        root_inline,
    );

    html! {
        <button class={classes![root_style]}>
            { for props.children.iter() }
        </button>
    }
}
