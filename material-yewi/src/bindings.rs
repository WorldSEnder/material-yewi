use gloo::timers::callback::Timeout;

#[derive(Debug)]
pub struct TimeoutHandle(Timeout);

pub fn set_timeout(handler: impl 'static + FnOnce(), delay_millis: u32) -> TimeoutHandle {
    TimeoutHandle(Timeout::new(delay_millis, handler))
}

pub fn clear_timeout(timeout: TimeoutHandle) {
    std::mem::forget(timeout)
}
