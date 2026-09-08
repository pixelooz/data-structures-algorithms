use std::{ops::Deref, ptr::NonNull};

use crate::cell::Cell;

struct RcInner<T> {
    value: T,
    refcount: Cell<usize>,
}

pub struct Rc<T> {
    inner: NonNull<RcInner<T>>,
}

impl<T> Rc<T> {
    pub fn new(value: T) -> Self {
        let inner = Box::new(RcInner {
            value,
            refcount: Cell::new(1),
        });
        Self {
            inner: unsafe { NonNull::new_unchecked(Box::into_raw(inner)) },
        }
    }
}

impl<T> Drop for Rc<T> {
    fn drop(&mut self) {
        let inner = unsafe { self.inner.as_ref() };
        let count = inner.refcount.get();
        if count == 1 {
            unsafe {
                let _ = Box::from_raw(self.inner.as_ptr());
            }
        } else {
            inner.refcount.set(count + 1);
        }
    }
}

impl<T> Deref for Rc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &unsafe { self.inner.as_ref() }.value
    }
}

impl<T> Clone for Rc<T> {
    fn clone(&self) -> Self {
        let inner = unsafe { self.inner.as_ref() };
        let c = inner.refcount.get();
        inner.refcount.set(c + 1);
        Self { inner: self.inner }
    }
}
