//#![feature(managed_boxes)]
#![feature(globs)]
#![allow(non_snake_case_functions)]
#![allow(dead_code)]
#![allow(unused_variable)]
#![allow(raw_pointer_deriving)]

use std::ptr;
use std::mem;

type Link<T> = Option<Box<Node<T>>>;
struct Rawlink<T> { p: *mut T }

/// Rawlink is a type like Option<T> but for holding a raw pointer
impl<T> Rawlink<T> {
    /// Like Option::None for Rawlink
    fn none() -> Rawlink<T> {
        Rawlink{p: ptr::mut_null()}
    }

    /// Like Option::Some for Rawlink
    fn some(n: &mut T) -> Rawlink<T> {
        Rawlink{p: n}
    }

    /// Convert the `Rawlink` into an Option value
    fn resolve_immut(&self) -> Option<&T> {
        unsafe { self.p.to_option() }
    }

    /// Convert the `Rawlink` into an Option value
    fn resolve(&mut self) -> Option<&mut T> {
        if self.p.is_null() {
            None
        } else {
            Some(unsafe { mem::transmute(self.p) })
        }
    }

    /// Return the `Rawlink` and replace with `Rawlink::none()`
    fn take(&mut self) -> Rawlink<T> {
        mem::replace(self, Rawlink::none())
    }
}





struct RawLink<T> { p : *mut T }

impl<T> RawLink<T> {
	/// Like Option::None
	fn none() -> RawLink<T> {
		RawLink{p: ptr::mut_null()}
	}

	/// Like Option::Some for RawLink
	fn some(n: &mut T) -> RawLink<T> {
		//RawLink{p: ptr::to_mut_unsafe_ptr(n)}
		unsafe { RawLink{p: mem::transmute(n)} }
	}

	fn resolve_immut(&self) -> Option<&T> {
		unsafe { self.p.to_option() }
	}
}


/*
	fn push_front_node(&mut self, mut new_head: Box<Node<T>>) {
		match self.list_head {
			None => {
				self.list_tail = RawLink::some(new_head);
				new_head.prev = RawLink::none();
				self.list_head = Some(new_head);
			}
			Some(ref mut head) => {
				new_head.prev = RawLink::none();
				head.prev = RawLink::some(new_head);
				mem::swap(head, &mut new_head);
				head.next = Some(new_head);
			}
		}
	}
*/
/*
impl<'self, A> DoubleEndedIterator<&'self A> for ListIterator<'self, A> {
	#[inline]
	fn next_back(&mut self) -> Option<&'self A> {
		if self.nelem == 0 {
			return None;
		}
		let tmp = self.tail.resolve_immut();
		do tmp.as_ref().map |prev| {
			self.nelem -= 1;
			self.tail = prev.prev;
			&prev.value
		}
	}
}*/


struct Node<T> {
    next: Link<T>,
    prev: Rawlink<Node<T>>,
    value: T,
}

#[deriving(Clone, Show)]
struct TreeNode {
	parent: Option<*mut TreeNode>,
	kids: Vec<Box<TreeNode>>
}

fn main() {
	let mut root = box TreeNode {
		parent: None,
		kids: vec!()
	};
	//let ref r2 = *root;
	let proot: *mut TreeNode = &mut*root;
	let k1 = box TreeNode {
		parent: Some(proot),
		kids: vec!()
	};
	root.kids.push(k1);
}

//impl TreeNode {
//	fn getParent<'a>(&'a self) -> &'a Option<Box<TreeNode>> { &(self.parent) }
//	//fn getParent(&self) -> Option<Box<TreeNode>> { self.parent.clone() }
//
//	fn getRoot<'a>(&'a self) -> &'a Box<TreeNode> {
//		match self.parent {
//			Some(ref p) => p.getRoot(),
//			None => *self
//		}
//	}
//
//	fn get<'a>(&'a self, ix: uint) -> &'a Box<TreeNode> {
//		self.kids.get(ix)
//	}
//
//	fn add(&mut self, kid: Box<TreeNode>) {
//		// kid.parent.remove(kid)
//		let p: *mut PTreeNode = &self;
//		kid.parent = Some(p);
//		self.kids.push(kid)
//	}
//	fn remove(&mut self) -> Box<TreeNode> {
//		let kid = self.kids.pop().unwrap();
//		//let oldparent = kid.getParent();
//		//oldparent.remove();
//		kid
//	}
//}
