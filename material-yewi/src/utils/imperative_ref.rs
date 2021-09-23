use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

#[derive(Clone)]
pub struct ImperativeRef<T>(Rc<RefCell<Inner<T>>>);

impl<T> Default for ImperativeRef<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<T> PartialEq for ImperativeRef<T> {
    fn eq(&self, other: &Self) -> bool {
        // equality for us means that the same binding is done
        Rc::ptr_eq(&self.0, &other.0)
    }
}

#[derive(PartialEq, Debug, Clone)]
struct Inner<T> {
    current: Option<T>,
}

impl<T> Default for Inner<T> {
    fn default() -> Self {
        Self { current: None }
    }
}

impl<T> ImperativeRef<T> {
    pub fn new() -> Self {
        <Self as Default>::default()
    }
    pub fn get(&self) -> Option<Ref<'_, T>> {
        let r = self.0.borrow();
        // feature(cell_filter_map) would simplify this and get rid of unwrap
        if r.current.is_some() {
            Some(Ref::map(r, |t| t.current.as_ref().unwrap()))
        } else {
            None
        }
    }
}

pub fn bind_imperative_ref<T>(r: &ImperativeRef<T>, t: T) {
    let mut i = r.0.borrow_mut();
    i.current = Some(t);
}
