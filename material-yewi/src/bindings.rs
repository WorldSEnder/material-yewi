use wasm_bindgen::{closure::Closure, prelude::wasm_bindgen};

#[derive(Debug)]
pub struct TimeoutHandle(i32, Closure<dyn FnMut()>);

impl Drop for TimeoutHandle {
    fn drop(&mut self) {
        do_clear_timeout(self.0);
    }
}

#[wasm_bindgen(inline_js = r#"
export function do_set_timeout(handler, delay) {
    return setTimeout(handler, delay);
}
export function do_clear_timeout(timeoutid) {
    clearTimeout(timeoutid);
}
"#)]
extern "C" {
    fn do_set_timeout(handler: &Closure<dyn FnMut()>, time: i32) -> i32;
    fn do_clear_timeout(timeoutid: i32);
}

pub fn set_timeout(handler: impl 'static + FnMut(), delay_millis: i32) -> TimeoutHandle {
    let closure = Closure::wrap(Box::new(handler) as Box<dyn FnMut()>);
    TimeoutHandle(do_set_timeout(&closure, delay_millis), closure)
}

pub fn clear_timeout(timeout: TimeoutHandle) {
    std::mem::forget(timeout)
}
