use material_styles_yew::{use_theme, Theme};
use stylist::ast::{sheet, Sheet};
use stylist::yew::use_style;
use yew::function_component;
use yew::html;
use yew::Html;
use yew::Properties;

use super::ripples::{KEYFRAMES_ENTER_NAME, KEYFRAMES_EXIT_NAME, KEYFRAMES_PULSATE_NAME};

#[derive(Debug, Clone, PartialEq)]
pub struct RippleStyleRoot {
    css_scopes: Sheet,
}

impl From<Sheet> for RippleStyleRoot {
    fn from(scopes: Sheet) -> Self {
        Self { css_scopes: scopes }
    }
}

pub const CLASS_RIPPLE: &str = "ripple";
pub const CLASS_VISIBLE: &str = "rippleVisible";
pub const CLASS_PULSATE: &str = "ripplePulsate";
pub const CLASS_CHILD: &str = "child";
pub const CLASS_CHILD_LEAVING: &str = "childLeaving";
pub const CLASS_CHILD_PULSATE: &str = "childPulsate";

struct ThemeStyles {
    root: Sheet,
}

fn derive_ripple_styles_from_theme(theme: &Theme) -> ThemeStyles {
    // FIXME: push into theme
    let ease_in_out = "cubic-bezier(0.4, 0, 0.2, 1)";
    const DURATION: &str = "550ms";
    const DURATION_SHORTER: &str = "200ms";
    const DURATION_SLOW: &str = "2500ms";

    let root_default = sheet!(
        opacity: 0;
        position: absolute;

        &.${CLASS_VISIBLE} {
            opacity: 0.3;
            transform: scale(1);
            animation-name: ${KEYFRAMES_ENTER_NAME};
            animation-duration: ${DURATION};
            animation-timing-function: ${ease_in_out};
        }

        &.${CLASS_PULSATE} {
            animation-duration: ${DURATION_SHORTER};
        }

        &${" "}*.${CLASS_CHILD} {
            opacity: 1;
            display: block;
            width: 100%;
            height: 100%;
            border-radius: 50%;
            background-color: currentColor;
        }

        &${" "}*.${CLASS_CHILD_LEAVING} {
            opacity: 0;
            animation-name: ${KEYFRAMES_EXIT_NAME};
            animation-duration: ${DURATION};
            animation-timing-function: ${ease_in_out};
        }

        &${" "}*.${CLASS_CHILD_PULSATE} {
            position: absolute;
            left: 0px;
            top: 0;
            animation-name: ${KEYFRAMES_PULSATE_NAME};
            animation-duration: ${DURATION_SLOW};
            animation-timing-function: ${ease_in_out};
            animation-iteration-count: infinite;
            animation-delay: ${DURATION_SHORTER};
        }
    );

    let root_override = theme
        .components
        .search_override::<RippleStyleRoot>()
        .map(|c| c.css_scopes.clone())
        .unwrap_or_default();

    let mut root = vec![];
    root.extend_from_slice(&root_default);
    root.extend_from_slice(&root_override);
    let root = Sheet::from(root);

    ThemeStyles { root }
}

#[derive(PartialEq, Properties, Debug, Clone)]
pub struct RippleProps {
    // position/size information
    pub pos_x: f64,
    pub pos_y: f64,
    pub pos_size: f64,
    // tell the component if it is supposed to be visible
    pub is_leaving: bool,
    pub is_pulsating: bool,
}

#[function_component]
pub fn Ripple(props: &RippleProps) -> Html {
    let styles = use_theme(derive_ripple_styles_from_theme);
    let root_class = use_style(/* "Mwi-ripple", */ styles.root.clone());

    let additional_style = format!(
        "width: {sz}px; height: {sz}px; top: {top}px; left: {left}px;",
        sz = props.pos_size,
        top = -(props.pos_size / 2.0) + props.pos_y,
        left = -(props.pos_size / 2.0) + props.pos_x,
    );
    let ripple_classes = yew::classes![
        root_class,
        CLASS_RIPPLE,
        CLASS_VISIBLE,
        props.is_pulsating.then(|| CLASS_PULSATE),
    ];
    let child_classes = yew::classes![
        CLASS_CHILD,
        props.is_pulsating.then(|| CLASS_CHILD_PULSATE),
        props.is_leaving.then(|| CLASS_CHILD_LEAVING),
    ];

    html! {
        <span class={ripple_classes} style={additional_style}>
            <span class={child_classes} />
        </span>
    }
}
