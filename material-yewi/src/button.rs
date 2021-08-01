use crate::memo::use_styles;
use css_in_rust::bindings::yew::use_scopes;
use css_in_rust::style::ast::Scopes;
use std::convert::TryInto;
use yew::classes;
use yew::function_component;
use yew::html;
use yew::virtual_dom::Attributes;
use yew::Callback;
use yew::Children;
use yew::KeyboardEvent;
use yew::MouseEvent;
use yew::Properties;
use yew_styles::Theme;

#[derive(Debug, Clone, PartialEq)]
pub struct ButtonStyleRoot {
    css_scopes: Scopes,
}

impl From<Scopes> for ButtonStyleRoot {
    fn from(scopes: Scopes) -> Self {
        Self { css_scopes: scopes }
    }
}

pub enum ButtonPressedEvent {
    MousePress(MouseEvent),
    EnterPress(KeyboardEvent),
    SpacebarPress(KeyboardEvent),
    // ... more later on?
}

#[derive(Default, Clone, PartialEq, Debug, Properties)]
pub struct ButtonProperties {
    #[prop_or_default]
    pub children: Children,
    /// Event fired when the button is considered pressed.
    /// This currently only includes clicking it, but would be expanded to keyboard events Enter and Spacebar
    ///  for screen reading compatibility. Also works correctly with Touch events on devices without a pointer.
    #[prop_or_default]
    pub on_pressed: Callback<ButtonPressedEvent>,
    #[prop_or_default]
    pub root_attrs: Attributes,
}

struct DefaultStyles {
    root_inline: Scopes,
    typography: Scopes,
    sizing: Scopes,
    hover: Scopes,
    root_override: Scopes,
}

fn derive_styles_from_theme(theme: Theme) -> DefaultStyles {
    let root_inline: Scopes = r#"
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

        -webkit-tap-highlight-color: transparent;
        -moz-appearance: none;
        -webkit-appearance: none;

        &::-moz-focus-inner {
            border-style: none;
        }
        "#
    .to_string()
    .try_into()
    .expect("error in css parsing");
    let typography = theme.typography.button.clone();
    let sizing = format!(
        r#"
        box-sizing: border-box;
        min-width: 64px;
        padding: 6px 8px;
        border-radius: {brdr};
        "#,
        brdr = theme.shape.border_radius
    )
    .try_into()
    .expect("error in css parsing");
    // TODO: use theme coloring
    let hover = r#"
        &:hover {
            text-decoration: none;
            background-color: rgba(25, 118, 210, 0.04);
        }
        "#
    .to_string()
    .try_into()
    .expect("error in css parsing");

    let root_override = theme
        .components
        .search_override::<ButtonStyleRoot>()
        .map(|c| &c.css_scopes)
        .cloned()
        .unwrap_or_default();

    DefaultStyles {
        root_inline,
        hover,
        sizing,
        typography,
        root_override,
    }
}

#[function_component(Button)]
pub fn button(props: &ButtonProperties) -> Html {
    let styles = use_styles(derive_styles_from_theme);

    let mut root_styles = Scopes::default();
    root_styles.append(styles.root_inline.clone());
    root_styles.append(styles.typography.clone());
    root_styles.append(styles.sizing.clone());
    root_styles.append(styles.hover.clone());
    root_styles.append(styles.root_override.clone());

    let root_style = use_scopes("Mwi-button-root", root_styles);
    let onclick = props.on_pressed.reform(ButtonPressedEvent::MousePress);

    html! {
        <button class={classes![&root_style]} onclick={onclick}>
            { for props.children.iter() }
        </button>
    }
}
