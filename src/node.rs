//#![feature(managed_boxes)]
#![feature(globs)]
#![allow(non_snake_case_functions)]
#![allow(dead_code)]
#![allow(unused_variable)]
#![allow(raw_pointer_deriving)]

use rawlink::{Rawlink};

mod rawlink;

//type Link<T> = Option<Box<Node<T>>>;
//
//struct Node<T> {
//    next: Link<T>,
//    prev: Rawlink<Node<T>>,
//    value: T,
//}

#[deriving(PartialEq, Clone, Show)]
struct TreeNode {
	parent: Rawlink<TreeNode>,
	kids: Vec<TreeNode>
}

impl TreeNode {

	//fn getParent<'a>(&'a self) -> &'a Option<TreeNode> { &(self.parent) }
	fn getParent(&mut self) -> Option<&mut TreeNode> { self.parent.resolve() }

	fn getRoot<'a>(&'a mut self) -> &'a mut TreeNode {
		match self.getParent() {
			Some(p) => p.getRoot(),
			None => self
		}
	}

	fn get<'a>(&'a self, ix: uint) -> &'a TreeNode {
		self.kids.get(ix)
	}
	fn get_mut<'a>(&'a mut self, ix: uint) -> &'a mut TreeNode {
		self.kids.get_mut(ix)
	}

	fn add(&mut self, mut kid: TreeNode) {
		match kid.getParent() {
			Some(oldparent) => { oldparent.remove(&mut kid); },
			None => {}
		}
		let p: *mut TreeNode = &mut*self;
		kid.parent = Rawlink::some(self);
		self.kids.push(kid)
	}
	fn indexOf(&mut self, kid: &TreeNode) -> Option<uint> {
		let mut ix = 0u;
		for it in self.kids.iter() {
			if it == kid { return Some(ix); }
			ix += 1
		}
		None
	}
	fn removeAt(&mut self, ix: uint) -> Option<TreeNode> {
		self.kids.remove(ix)
	}
	fn remove(&mut self, kid: &mut TreeNode) -> Option<TreeNode> {
		//let ref k = *kid;
		match self.indexOf(kid) {
			Some(ix) => self.removeAt(ix),
			None => None
		}
	}
    fn removeFromOwner(&mut self) {
	    match self.getParent() {
	    	Some(oldOwner) => {
	    		oldOwner.remove(self);
	    		self.onAdd(None);
	    	},
	    	None => {}
	    }
    }
    fn onAdd(&self, kid: Option<&TreeNode>) {}
}


fn main() {
	let mut root = TreeNode {
		parent: Rawlink::none(),
		kids: vec!()
	};
	let  k1 = TreeNode {
		parent: Rawlink::some(&mut root),
		kids: vec!()
	};
	root.add(k1);
	println!("{}", root);
	//let k = root.get(0);
	match root.removeAt(0) {
		Some(k) => println!("{}, {}", root, k),
		None => println!("woops")
	}
	//println!("{}, {}", root, k1);
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
