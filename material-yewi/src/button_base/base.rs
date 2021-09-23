use material_styles_yew::use_theme;
use material_styles_yew::Theme;
use stylist::ast::{sheet, Sheet};
use stylist::yew::use_style;
use web_sys::FocusEvent;
use yew::classes;
use yew::function_component;
use yew::html;
use yew::use_effect_with_deps;
use yew::use_ref;
use yew::use_state;
use yew::Callback;
use yew::Children;
use yew::KeyboardEvent;
use yew::MouseEvent;
use yew::NodeRef;
use yew::Properties;
use yew::TouchEvent;

use super::ripples::*;
use crate::utils::imperative_ref::ImperativeRef;
use crate::utils::use_focus_visible::use_focus_visible;
use crate::utils::use_focus_visible::FocusVisibleHandle;
use crate::utils::use_state_ext::UseStateHandleExt;

#[derive(Debug, Clone, PartialEq)]
pub struct ButtonBaseStyleRoot {
    css_scopes: Sheet,
}

impl From<Sheet> for ButtonBaseStyleRoot {
    fn from(scopes: Sheet) -> Self {
        Self { css_scopes: scopes }
    }
}

pub const CLASS_DISABLED: &str = "disabled";
pub const CLASS_FOCUS_VISIBLE: &str = "focus";

pub enum ButtonPressedEvent {
    MousePress(MouseEvent),
    EnterPress(KeyboardEvent),
    SpacebarPress(KeyboardEvent),
    // ... more later on?
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum RippleBehaviour {
    Disabled,
    Centered,
    Interactive,
}

#[derive(Clone, PartialEq, Debug, Properties)]
pub struct ButtonBaseProperties {
    #[prop_or_default]
    pub class: Sheet,
    /// Event fired when the button is considered pressed.
    /// This currently only includes clicking it, but would be expanded to keyboard events Enter and Spacebar
    ///  for screen reading compatibility. Also works correctly with Touch events on devices without a pointer.
    #[prop_or_default]
    pub on_pressed: Callback<ButtonPressedEvent>,
    #[prop_or(false)]
    pub disabled: bool,
    #[prop_or(RippleBehaviour::Interactive)]
    pub ripples: RippleBehaviour,
    #[prop_or(0)]
    pub tab_index: i32,
    // TODO: additional properties
    #[prop_or_default]
    pub children: Children,
}

struct ThemeStyles {
    root: Sheet,
}

fn derive_styles_from_theme(theme: Theme) -> ThemeStyles {
    let root_default = sheet!(
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
        &.${CLASS_DISABLED} {
            pointer-events: none;
            cursor: default;
        }
        @media print {
            color-adjust: exact;
        }
    );

    let root_override = theme
        .components
        .search_override::<ButtonBaseStyleRoot>()
        .map(|c| &c.css_scopes)
        .cloned()
        .unwrap_or_default();

    let mut root = vec![];
    root.extend_from_slice(&root_default);
    root.extend_from_slice(&root_override);
    let root = Sheet::from(root);

    ThemeStyles { root }
}

trait RippleHandleAction<Arg> {
    fn handle(h: &RipplesHandle, arg: Arg);
}
struct RippleActionStart;
impl RippleHandleAction<MouseEvent> for RippleActionStart {
    fn handle(h: &RipplesHandle, ev: MouseEvent) {
        h.start.emit(RippleStartReason::MousePress(ev))
    }
}
impl RippleHandleAction<TouchEvent> for RippleActionStart {
    fn handle(h: &RipplesHandle, ev: TouchEvent) {
        h.start.emit(RippleStartReason::TouchStart(ev))
    }
}
struct RippleActionStop;
impl<Arg> RippleHandleAction<Arg> for RippleActionStop {
    fn handle(h: &RipplesHandle, _: Arg) {
        h.stop.emit(());
    }
}

fn use_handle<Arg: 'static, Action: RippleHandleAction<Arg>>(
    handle: &ImperativeRef<RipplesHandle>,
    _action: Action,
    event_callback: impl 'static + Fn(&Arg),
) -> Callback<Arg> {
    let handle = handle.clone();
    Callback::from(move |v| {
        event_callback(&v);

        let inner_handle = handle.get().as_deref().cloned();
        if let Some(h) = inner_handle {
            Action::handle(&h, v);
        }
    })
}

#[function_component(ButtonBase)]
pub fn button_base(props: &ButtonBaseProperties) -> Html {
    let styles = use_theme(derive_styles_from_theme);
    let mut root_sheet = vec![];
    root_sheet.extend_from_slice(&styles.root);
    root_sheet.extend_from_slice(&props.class);
    let root_style = use_style(/* "Mwi-button-base" */ Sheet::from(root_sheet));

    let button_ref = use_ref(NodeRef::default);
    let ripples_handle = use_ref(ImperativeRef::<RipplesHandle>::new);

    let FocusVisibleHandle {
        // This focus_visible provides tracking, but is not our source of truth
        is_visible: tracked_focus_visible,
        onblur: handle_blur_visible,
        onfocus: handle_focus_visible,
    } = use_focus_visible(button_ref.borrow().clone());
    let focus_visible = use_state(|| false);

    if props.disabled {
        focus_visible.relaxed_set(false);
    }

    let ripples_handle_capture = ripples_handle.clone();
    use_effect_with_deps(
        move |focus_visible| {
            if *focus_visible {
                if let Some(h) = (*ripples_handle_capture.borrow()).get().as_deref() {
                    h.start.emit(RippleStartReason::FocusVisible);
                }
            }
            || {}
        },
        *focus_visible,
    );

    let handle_click = props.on_pressed.reform(ButtonPressedEvent::MousePress);

    let handle_mouse_down = use_handle(&*ripples_handle.borrow(), RippleActionStart, |_| {});
    let handle_touch_start = use_handle(&*ripples_handle.borrow(), RippleActionStart, |_| {});

    // TODO: figure out why we'd need that
    // let handle_drag_leave = use_handle(&*ripples_handle.borrow(), RippleActionStop, |_| {});
    let handle_context_menu = use_handle(&*ripples_handle.borrow(), RippleActionStop, |_| {});
    let handle_mouse_up = use_handle(&*ripples_handle.borrow(), RippleActionStop, |_| {});
    let focus_visible_capture = focus_visible.clone();
    let handle_mouse_out = use_handle(
        &*ripples_handle.borrow(),
        RippleActionStop,
        move |ev: &MouseEvent| {
            if *focus_visible_capture {
                ev.prevent_default();
            }
        },
    );
    let handle_touch_end = use_handle(&*ripples_handle.borrow(), RippleActionStop, |_| {});
    let handle_touch_move = use_handle(&*ripples_handle.borrow(), RippleActionStop, |_| {});

    let focus_visible_capture = focus_visible.clone();
    let tracked_focus_visible_capture = tracked_focus_visible.clone();
    let handle_blur = use_handle(
        &*ripples_handle.borrow(),
        RippleActionStop,
        move |ev: &FocusEvent| {
            handle_blur_visible.emit(ev.clone());
            focus_visible_capture.relaxed_set(*tracked_focus_visible_capture.borrow());
        },
    );
    let focus_visible_capture = focus_visible.clone();
    let tracked_focus_visible_capture = tracked_focus_visible;
    // TODO: replace with use_memo?
    let handle_focus = use_state(|| {
        Callback::from(move |ev: FocusEvent| {
            handle_focus_visible.emit(ev);
            focus_visible_capture.relaxed_set(*tracked_focus_visible_capture.borrow());
        })
    });
    // TODO: handleKeyDown
    // TODO: handleKeyUp

    let classes = classes![
        root_style,
        props.disabled.then(|| CLASS_DISABLED),
        focus_visible.then(|| CLASS_FOCUS_VISIBLE),
    ];
    let button_ref = button_ref.borrow().clone();
    let ripples_handle = ripples_handle.borrow().clone();
    html! {
        <button
            ref={button_ref}
            class={classes}
            onfocusout={handle_blur}
            onclick={handle_click}
            oncontextmenu={handle_context_menu}
            onfocusin={(*handle_focus).clone()}
            // onkeydown={handle_key_down}
            // onkeyup={handle_key_up}
            onmousedown={handle_mouse_down}
            onmouseout={handle_mouse_out}
            onmouseup={handle_mouse_up}
            // ondragleave={handle_drag_leave}
            ontouchend={handle_touch_end}
            ontouchmove={handle_touch_move}
            ontouchstart={handle_touch_start}
            tabindex={(if props.disabled { -1 } else { props.tab_index }).to_string()}
        >
            { for props.children.iter() }
            <Ripples handle={ripples_handle} />
        </button>
    }
}
