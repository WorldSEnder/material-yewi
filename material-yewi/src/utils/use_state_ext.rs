use std::ops::Deref;
use yew::functional::UseStateHandle;

pub trait UseStateHandleExt<T> {
    /// Set the value if the current value is not already equal to the value
    fn relaxed_set(&self, value: T);
}

impl<T: PartialEq> UseStateHandleExt<T> for UseStateHandle<T> {
    fn relaxed_set(&self, value: T) {
        if self.deref() != &value {
            self.set(value);
        }
    }
}
