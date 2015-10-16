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

/// Get at the internals.
#[inline(always)]
pub unsafe fn as_raw<T>(r: &rc::Rc<T>) -> *mut RcBox<T> {
    let internal_rc_ref: &Rc<T> = mem::transmute(r);
    *internal_rc_ref._ptr
}

/// For use only if you know that the `Rc` is unique. We bypass
/// checking for that.
#[inline(always)]
pub unsafe fn to_value_ptr<T>(r: &mut rc::Rc<T>) -> *mut T {
    let box_ptr = as_raw(r);
    let value_ref: &mut T = &mut (*box_ptr).value;
    value_ref as *mut T
}

/// A safe converter that does a runtime check for uniqueness.
/// Useful for mutating within an `Rc` after construction unsafely
/// to keep the type of the container immutable.
#[allow(dead_code)]
#[inline(always)]
pub fn safe_to_value_ptr<T>(r: &mut rc::Rc<T>) -> *mut T {
    rc::Rc::get_mut(r).unwrap() as *mut T
}
