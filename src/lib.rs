//! Immmutable, persistent list as in FP languages.

// For unsafe `Rc` hacking.
#![feature(nonzero)]
#![feature(core)]

extern crate core;

mod rc_utils;
pub mod list;
