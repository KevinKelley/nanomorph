//#![feature(managed_boxes)]
#![feature(globs)]
#![allow(non_snake_case_functions)]
#![allow(dead_code)]
#![allow(unused_variable)]
#![allow(raw_pointer_deriving)]

use std::ptr;
use std::mem;

type Link<T> = Option<Box<Node<T>>>;

#[deriving(Clone, Show)]
struct Rawlink<T> { p: *mut T }

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

struct Node<T> {
    next: Link<T>,
    prev: Rawlink<Node<T>>,
    value: T,
}

#[deriving(Clone, Show)]
struct TreeNode {
	parent: Rawlink<TreeNode>,
	kids: Vec<Box<TreeNode>>
}

impl TreeNode {

	//fn getParent<'a>(&'a self) -> &'a Option<Box<TreeNode>> { &(self.parent) }
	fn getParent(&mut self) -> Option<&mut TreeNode> { self.parent.resolve() }

	fn getRoot<'a>(&'a mut self) -> &'a mut TreeNode {
		match self.getParent() {
			Some(p) => p.getRoot(),
			None => self
		}
	}

	fn get<'a>(&'a self, ix: uint) -> &'a Box<TreeNode> {
		self.kids.get(ix)
	}

	fn add(&mut self, mut kid: Box<TreeNode>) {
		// kid.parent.remove(kid)
		let p: *mut TreeNode = &mut*self;
		kid.parent = Rawlink::some(self);
		self.kids.push(kid)
	}
	fn remove(&mut self) -> Box<TreeNode> {
		let kid = self.kids.pop().unwrap();
		//let oldparent = kid.getParent();
		//oldparent.remove();
		kid
	}
}


fn main() {
	let mut root = box TreeNode {
		parent: Rawlink::none(),
		kids: vec!()
	};
	let k1 = box TreeNode {
		parent: Rawlink::some(&mut*root),
		kids: vec!()
	};
	root.kids.push(k1);

	println!("{}", root);
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
