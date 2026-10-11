use std::marker::PhantomData;

pub struct RefWithFlag<'a, T> {
    ptr_and_bit: usize,
    behaves_like: PhantomData<&'a T>,
}

impl<'a, T: 'a> RefWithFlag<'a, T> {
    pub fn new(ptr: &'a T, flag: bool) -> RefWithFlag<'a, T> {
        assert!(align_of::<T>().is_multiple_of(2));
        RefWithFlag {
            ptr_and_bit: ptr as *const T as usize | flag as usize,
            behaves_like: PhantomData,
        }
    }
}
