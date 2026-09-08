use std::ops::Deref;

use crate::cell::Cell;

struct RcInner<T> {
    value: T,
    refcount: Cell<usize>,
}

pub struct Rc<T> {
    inner: *const RcInner<T>,
}

// For drop impl of this type, we'll basically check if the `refcount` is 1, means we are the only
// ones left, and are being dropped, meaning the memory should be freed.
// Otherwise, if there are any more than 1 `refcount` we don't free the memory.

impl<T> Rc<T> {
    pub fn new(value: T) -> Self {
        let inner = Box::new(RcInner {
            value,
            refcount: Cell::new(1),
        });
        Self {
            inner: Box::into_raw(inner),
        }
    }
}

impl<T> Deref for Rc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &unsafe { &*self.inner }.value
    }
}

impl<T> Clone for Rc<T> {
    fn clone(&self) -> Self {
        let inner = unsafe { &*self.inner };
        let c = inner.refcount.get();
        inner.refcount.set(c + 1);
        Self { inner }
    }
}
