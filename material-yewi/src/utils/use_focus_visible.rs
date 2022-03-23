use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use web_sys::Document;
use web_sys::KeyboardEvent;
use web_sys::Node;
use web_sys::VisibilityState;
use web_sys::{HtmlElement, HtmlInputElement, HtmlTextAreaElement};
use yew::use_effect_with_deps;
use yew::use_mut_ref;
use yew::Callback;
use yew::FocusEvent;
use yew::NodeRef;

use crate::bindings::{clear_timeout, set_timeout, TimeoutHandle};

pub struct FocusVisibleHandle {
    pub onblur: Callback<FocusEvent>,
    pub onfocus: Callback<FocusEvent>,
    pub is_visible: Rc<RefCell<bool>>,
}

struct GlobalHackState {
    had_keyboard_event: bool,
    had_focus_visible_recently: bool,
    focus_recently_timeout: Option<TimeoutHandle>,
}

impl GlobalHackState {
    fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            had_keyboard_event: false,
            had_focus_visible_recently: false,
            focus_recently_timeout: None,
        }))
    }
    fn set_had_focus_recently(state: &Rc<RefCell<Self>>) {
        let captured_self = state.clone();
        let mut self_ = state.borrow_mut();
        if let Some(t) = self_.focus_recently_timeout.take() {
            clear_timeout(t);
        }
        self_.had_focus_visible_recently = true;
        self_.focus_recently_timeout = Some(set_timeout(
            move || captured_self.borrow_mut().had_focus_visible_recently = false,
            100,
        ));
    }
}

struct GlobalFocusVisibleHack {
    state: Rc<RefCell<GlobalHackState>>,
    doc: Document,
    on_keydown: Closure<dyn FnMut(KeyboardEvent)>,
    on_pointerdown: Closure<dyn FnMut()>,
    on_visiblitychange: Closure<dyn FnMut()>,
}

impl GlobalFocusVisibleHack {
    fn new(doc: Document) -> Self {
        let state = GlobalHackState::new();
        let state_capture = state.clone();
        let on_keydown = Closure::wrap(Box::new(move |ev: KeyboardEvent| {
            if !ev.meta_key() && !ev.ctrl_key() && !ev.alt_key() {
                state_capture.borrow_mut().had_keyboard_event = true;
            }
        }) as Box<dyn FnMut(_)>);

        let state_capture = state.clone();
        let on_pointerdown = Closure::wrap(Box::new(move || {
            state_capture.borrow_mut().had_keyboard_event = false;
        }) as Box<dyn FnMut()>);

        let doc_capture = doc.clone();
        let state_capture = state.clone();
        let on_visiblitychange = Closure::wrap(Box::new(move || {
            let mut state = state_capture.borrow_mut();
            if doc_capture.visibility_state() == VisibilityState::Hidden
                && state.had_focus_visible_recently
            {
                state.had_keyboard_event = true;
            }
        }) as Box<dyn FnMut()>);

        doc.add_event_listener_with_callback("onkeydown", on_keydown.as_ref().unchecked_ref())
            .unwrap();
        doc.add_event_listener_with_callback("mousedown", on_pointerdown.as_ref().unchecked_ref())
            .unwrap();
        doc.add_event_listener_with_callback(
            "pointerdown",
            on_pointerdown.as_ref().unchecked_ref(),
        )
        .unwrap();
        doc.add_event_listener_with_callback("touchstart", on_pointerdown.as_ref().unchecked_ref())
            .unwrap();
        doc.add_event_listener_with_callback(
            "visibilitychange",
            on_visiblitychange.as_ref().unchecked_ref(),
        )
        .unwrap();
        Self {
            state,
            doc,
            on_keydown,
            on_pointerdown,
            on_visiblitychange,
        }
    }

    fn is_focus_visible(&self, target: &HtmlElement) -> bool {
        self.state.borrow().had_keyboard_event || {
            // Some elements are always supposed to be :focus_visible
            if let Some(target_input) = target.dyn_ref::<HtmlInputElement>() {
                const TYPE_WHITELIST: [&str; 13] = [
                    "text",
                    "search",
                    "url",
                    "tel",
                    "email",
                    "password",
                    "number",
                    "date",
                    "month",
                    "week",
                    "time",
                    "datetime",
                    "datetime-local",
                ];
                TYPE_WHITELIST.contains(&target_input.type_().as_str()) && !target_input.read_only()
            } else if let Some(target_area) = target.dyn_ref::<HtmlTextAreaElement>() {
                !target_area.read_only()
            } else {
                target.is_content_editable()
            }
        }
    }
}

impl Drop for GlobalFocusVisibleHack {
    fn drop(&mut self) {
        let Self {
            doc,
            on_keydown,
            on_pointerdown,
            on_visiblitychange,
            ..
        } = self;

        doc.remove_event_listener_with_callback("onkeydown", on_keydown.as_ref().unchecked_ref())
            .unwrap();
        doc.remove_event_listener_with_callback(
            "mousedown",
            on_pointerdown.as_ref().unchecked_ref(),
        )
        .unwrap();
        doc.remove_event_listener_with_callback(
            "pointerdown",
            on_pointerdown.as_ref().unchecked_ref(),
        )
        .unwrap();
        doc.remove_event_listener_with_callback(
            "touchstart",
            on_pointerdown.as_ref().unchecked_ref(),
        )
        .unwrap();
        doc.remove_event_listener_with_callback(
            "visibilitychange",
            on_visiblitychange.as_ref().unchecked_ref(),
        )
        .unwrap();
    }
}

#[derive(Clone)]
struct DocumentHandle(Rc<RefCell<Option<GlobalFocusVisibleHack>>>);
impl DocumentHandle {
    fn is_focus_visible(&self, event: &FocusEvent) -> bool {
        let target = event.target().unwrap().dyn_into::<HtmlElement>().unwrap();
        // focus visible event always have an Element as target
        if let Ok(true) = target.matches(":focus-visible") {
            return true;
        }
        if let Some(ref global_hook) = *self.0.borrow() {
            global_hook.is_focus_visible(&target)
        } else {
            // Otherwise return false, maybe panic here?
            false
        }
    }
}

fn setup(handle: DocumentHandle, node: Node) -> impl FnOnce() {
    let doc = node.owner_document().expect("node contained in a document");
    // TODO: cache per document, then drop in returned FnOnce
    // TODO: can skip this setup if :focus-visible is supported
    *handle.0.borrow_mut() = Some(GlobalFocusVisibleHack::new(doc));
    move || {}
}

#[yew::hook]
pub fn use_focus_visible(node: &NodeRef) -> FocusVisibleHandle {
    let is_visible = use_mut_ref(|| false);
    let doc_handle = DocumentHandle(Rc::default());

    let doc_handle_capture = doc_handle.clone();
    use_effect_with_deps(
        |node| {
            let teardown = node.get().map(|n| setup(doc_handle_capture, n));
            || {
                if let Some(t) = teardown {
                    t()
                }
            }
        },
        node.clone(),
    );

    let is_visible_capture_focus = is_visible.clone();
    let is_visible_capture_blur = is_visible.clone();
    let doc_handle_capture = doc_handle.clone();
    let onblur = Callback::from(move |_| {
        if std::mem::replace(&mut is_visible_capture_blur.borrow_mut(), false) {
            if let Some(ref global) = *doc_handle_capture.0.borrow() {
                GlobalHackState::set_had_focus_recently(&global.state);
            }
        }
    });
    let onfocus = Callback::from(move |ev| {
        if doc_handle.is_focus_visible(&ev) {
            *is_visible_capture_focus.borrow_mut() = true;
        }
    });
    FocusVisibleHandle {
        onblur,
        onfocus,
        is_visible,
    }
}
