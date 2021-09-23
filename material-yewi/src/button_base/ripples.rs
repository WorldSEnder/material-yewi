use std::rc::Rc;

use lazy_static::lazy_static;
use material_styles_yew::use_theme;
use material_styles_yew::Theme;
use stylist::ast::{sheet, Sheet};
use stylist::yew::{use_style, Global};
use yew::classes;
use yew::function_component;
use yew::use_reducer;
use yew::use_ref;
use yew::web_sys::Element;
use yew::Callback;
use yew::MouseEvent;
use yew::NodeRef;
use yew::Properties;
use yew::TouchEvent;
use yew::{html, html_nested};

use super::ripple::{Ripple, RippleProps};
use crate::utils::imperative_ref::{bind_imperative_ref, ImperativeRef};

// FIXME: keyframes should be uniquely named not use hacky names
pub const KEYFRAMES_ENTER_NAME: &str = "__enter_mwi_ripples_anim";
pub const KEYFRAMES_EXIT_NAME: &str = "__exit_mwi_ripples_anim";
pub const KEYFRAMES_PULSATE_NAME: &str = "__pulsate_mwi_ripples_anim";
lazy_static! {
    static ref KEYFRAMES_ENTER_SHEET: Sheet = sheet!(
        "@keyframes ${name} {
            0% {
                transform: scale(0);
                opacity: 0.1;
            }
            100% {
                transform: scale(1);
                opacity: 0.3;
            }
        }",
        name = KEYFRAMES_ENTER_NAME
    );
    static ref KEYFRAMES_EXIT_SHEET: Sheet = sheet!(
        "@keyframes ${name} {
            0% {
                opacity: 1;
            }
            100% {
                opacity: 0;
            }
        }",
        name = KEYFRAMES_EXIT_NAME
    );
    static ref KEYFRAMES_PULSATE_SHEET: Sheet = sheet!(
        "@keyframes ${name} {
            0% {
                transform: scale(1);
            }
            50% {
                transform: scale(0.92);
            }
            100% {
                transform: scale(1);
            }
        }",
        name = KEYFRAMES_PULSATE_NAME
    );
}

#[derive(Debug, Clone, PartialEq)]
pub struct RipplesStyleRoot {
    css_scopes: Sheet,
}

impl From<Sheet> for RipplesStyleRoot {
    fn from(scopes: Sheet) -> Self {
        Self { css_scopes: scopes }
    }
}

struct ThemeStyles {
    root: Sheet,
}

fn derive_styles_from_theme(theme: Theme) -> ThemeStyles {
    let root_default = sheet!(
        overflow: hidden;
        pointer-events: none;
        position: absolute;
        z-index: 0;
        top: 0;
        right: 0;
        bottom: 0;
        left: 0;
        border-radius: inherit;
    );

    let root_override = theme
        .components
        .search_override::<RipplesStyleRoot>()
        .map(|c| c.css_scopes.clone())
        .unwrap_or_default();

    let mut root = vec![];
    root.extend_from_slice(&root_default);
    root.extend_from_slice(&root_override);
    let root = Sheet::from(root);

    ThemeStyles { root }
}

#[derive(Debug)]
pub enum RippleStartReason {
    MousePress(MouseEvent),
    TouchStart(TouchEvent),
    FocusVisible,
}

impl RippleStartReason {
    fn to_params(&self, node: &NodeRef) -> RippleProps {
        let element = node.cast::<Element>();
        let rect = element.as_ref().map(|e| e.get_bounding_client_rect());
        let (_x, _y, width, height, left, top) = match rect {
            Some(ref r) => (r.x(), r.y(), r.width(), r.height(), r.left(), r.top()),
            None => (0., 0., 0., 0., 0., 0.),
        };
        let pulsate = matches!(self, RippleStartReason::FocusVisible);
        let center = pulsate; // TODO: || props.center;

        let (pos_x, pos_y) = match self {
            Self::TouchStart(ref ev_touch) if !center => {
                let first_touch = ev_touch.touches().get(0).unwrap();
                (
                    first_touch.client_x() as f64 - left,
                    first_touch.client_y() as f64 - top,
                )
            }
            Self::MousePress(ref ev_press)
                if !center && (ev_press.client_x() != 0 || ev_press.client_y() != 0) =>
            {
                (
                    ev_press.client_x() as f64 - left,
                    ev_press.client_y() as f64 - top,
                )
            }
            _ => ((width / 2.).round(), (height / 2.).round()),
        };
        let pos_size = if center {
            ((2. * width * width + height * height) / 3.).sqrt()
        } else {
            let (client_x, client_y) = match element {
                Some(ref e) => (e.client_width(), e.client_height()),
                None => (0, 0),
            };
            let size_x = 2. + 2. * pos_x.max((client_x as f64 - pos_x).abs());
            let size_y = 2. + 2. * pos_y.max((client_y as f64 - pos_y).abs());
            (size_x * size_x + size_y * size_y).sqrt()
        };

        yew::props! {RippleProps {
            pos_x,
            pos_y,
            pos_size,
            is_pulsating: pulsate,
            is_leaving: false,
        }}
    }
}

#[derive(Clone)]
pub struct RipplesHandle {
    pub start: Callback<RippleStartReason>,
    pub stop: Callback<()>,
}

#[derive(PartialEq, Properties)]
pub struct RipplesProp {
    pub handle: ImperativeRef<RipplesHandle>,
}

#[function_component(Ripples)]
pub fn ripples(props: &RipplesProp) -> Html {
    type RippleEntry = (u32, RippleProps);

    let id_counter = use_ref(|| 0u32);
    let container = use_ref(NodeRef::default);
    let ripples = use_reducer(
        |mut state, act: Box<dyn FnOnce(&mut Vec<RippleEntry>)>| {
            Rc::make_mut(&mut state);
            let mut new_state = Rc::try_unwrap(state).expect("just made unique");
            act(&mut new_state);
            new_state
        },
        vec![],
    );

    let container_capture = container.clone();
    let ripples_capture_start = ripples.clone();
    let ripples_capture_stop = ripples.clone();
    let handles = RipplesHandle {
        start: Callback::from(move |event: RippleStartReason| {
            let id_capture = id_counter.clone();
            let container_capture = container_capture.clone();
            ripples_capture_start.dispatch(Box::new(move |v| {
                let next_id = {
                    let mut c = id_capture.borrow_mut();
                    *c += 1;
                    *c
                };
                let ripple_params = event.to_params(&*container_capture.borrow());
                v.push((next_id, ripple_params));
            }));
        }),
        stop: Callback::from(move |_: ()| {
            let ripples_capture_stop = ripples_capture_stop.clone();
            let ripples_capture = ripples_capture_stop.clone();
            ripples_capture_stop.dispatch(Box::new(move |v| {
                let leaving_ripple = match v.iter_mut().find(|v| !v.1.is_leaving) {
                    None => return,
                    Some(r) => r,
                };
                leaving_ripple.1.is_leaving = true;
                let ripple_id = leaving_ripple.0;
                gloo::timers::callback::Timeout::new(550, move || {
                    ripples_capture.dispatch(Box::new(move |v| {
                        if let Some(p) = v.iter().position(|v| v.0 == ripple_id) {
                            v.swap_remove(p);
                        }
                    }));
                })
                .forget();
            }));
        }),
    };
    bind_imperative_ref(&props.handle, handles);

    let themed = use_theme(derive_styles_from_theme);
    let style = use_style(/* "Mwi-ripple-host", */ themed.root.clone());

    let container = container.borrow().clone();
    html! {
        <span class={classes![style]} ref={container}>
            <Global key="sheet_enter" css={KEYFRAMES_ENTER_SHEET.clone()}/>
            <Global key="sheet_exit" css={KEYFRAMES_EXIT_SHEET.clone()}/>
            <Global key="sheet_pulsate" css={KEYFRAMES_PULSATE_SHEET.clone()}/>
            {
                for ripples.iter().map(|(k, r)|
                    html_nested! {
                        <Ripple key={*k} ..r.clone() />
                    }
                )
            }
        </span>
    }
}
