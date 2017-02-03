//! Utilities for `Rc`, some of them unsafe!

use std::rc;

/// For unsafe memory hacking.
use std::mem;

use std::cell::Cell;

/// Evil: copied from [source](https://doc.rust-lang.org/src/alloc/up/src/liballoc/rc.rs.html#174-178).
#[allow(dead_code)]
pub struct RcBox<T: ?Sized> {
    strong: Cell<usize>,
    weak: Cell<usize>,
    value: T,
}

/// Evil: copied from [source](https://doc.rust-lang.org/src/alloc/up/src/liballoc/rc.rs.html#186-190) except omitting the `Shared` and just faking a raw pointer.
struct Rc<T: ?Sized> {
    _ptr: *mut RcBox<T>,
}

/// Get the raw pointer stored inside an `Rc`.
#[inline(always)]
unsafe fn as_raw<T>(r: &rc::Rc<T>) -> *mut RcBox<T> {
    mem::transmute::<&rc::Rc<T>, &Rc<T>>(r)
        ._ptr
}

/// For use only if you know that the `Rc` is unique.
#[inline(always)]
pub unsafe fn get_mut<T>(r: &mut rc::Rc<T>) -> *mut T {
    &mut (*as_raw(r)).value
}

/// The safe version of `get_mut` does a reference count check.
#[allow(dead_code)]
#[inline(always)]
pub fn get_mut_unwrap<T>(r: &mut rc::Rc<T>) -> *mut T {
    rc::Rc::get_mut(r).unwrap()
}
