//! Utilities for `Rc`, some of them unsafe!

use std::rc;

/// For unsafe memory hacking.
use std::mem;

use core::nonzero::NonZero;
use std::cell::Cell;

/// Evil: copied from [source](https://doc.rust-lang.org/nightly/src/alloc/rc.rs.html#171).
#[allow(dead_code)]
pub struct RcBox<T: ?Sized> {
    strong: Cell<usize>,
    weak: Cell<usize>,
    value: T
}

/// Evil: copied from [source](https://doc.rust-lang.org/nightly/src/alloc/rc.rs.html#183).
struct Rc<T: ?Sized> {
    _ptr: NonZero<*mut RcBox<T>>,
}

/// Whether two `Rc` are the same pointer underneath.
#[inline(always)]
pub fn eq<T>(r1: &rc::Rc<T>, r2: &rc::Rc<T>) -> bool {
    unsafe {
        as_raw(r1) == as_raw(r2)
    }
}

/// Get the raw pointer stored inside an `Rc`.
#[inline(always)]
unsafe fn as_raw<T>(r: &rc::Rc<T>) -> *mut RcBox<T> {
    *mem::transmute::<&rc::Rc<T>, &Rc<T>>(r)
        ._ptr
}

/// For use only if you know that the `Rc` is unique.
#[inline(always)]
pub unsafe fn get_mut<T>(r: &mut rc::Rc<T>) -> *mut T {
    &mut (*as_raw(r)).value
}

/// The safe version of `get_mut_unsafe`.
#[allow(dead_code)]
#[inline(always)]
pub fn get_mut_unwrap<T>(r: &mut rc::Rc<T>) -> *mut T {
    rc::Rc::get_mut(r).unwrap()
}
