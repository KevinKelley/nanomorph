#![allow(raw_pointer_deriving)]

use std::ptr;
use std::mem;


#[deriving(PartialEq, Clone, Show)]
pub struct Rawlink<T> { p: *mut T }

/// Rawlink is a type like Option<T> but for holding a raw pointer
impl<T> Rawlink<T> {
    /// Like Option::None for Rawlink
    pub fn none() -> Rawlink<T> {
        Rawlink{p: ptr::mut_null()}
    }

    /// Like Option::Some for Rawlink
    pub fn some(n: &mut T) -> Rawlink<T> {
        Rawlink{p: n}
    }

    /// Convert the `Rawlink` into an Option value
    pub fn resolve_immut(&self) -> Option<&T> {
        unsafe { self.p.to_option() }
    }

    /// Convert the `Rawlink` into an Option value
    pub fn resolve(&mut self) -> Option<&mut T> {
        if self.p.is_null() {
            None
        } else {
            Some(unsafe { mem::transmute(self.p) })
        }
    }

    /// Return the `Rawlink` and replace with `Rawlink::none()`
    pub fn take(&mut self) -> Rawlink<T> {
        mem::replace(self, Rawlink::none())
    }
}
