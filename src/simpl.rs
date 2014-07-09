#![crate_id = "nanomorph#0.1"]
#![desc = "a tiny morphic implementation on nanoVG"]
#![license = "MIT"]
#![crate_type = "lib"]

#![feature(globs)]
#![allow(non_snake_case_functions)]
#![allow(dead_code)]
#![allow(unused_variable)]

pub use gfx::*;
    use rawlink::*;

pub mod gfx;
    mod rawlink;

/////////////////////////////////////////////
#[deriving(PartialEq)] //Eq Clone Show Hash PartialEq
struct Morphic {
    owner: Option<Box<Morphic>>,
    subMorphs: Vec<Box<Morphic>>,
}
//impl PartialEq for Box<Morphic> {
//    fn eq(&self, other: &Box<Morphic>) -> bool { *self == *other }
//}

impl Morphic {
	// Collection
	fn numItems(&self) -> uint { self.subMorphs.len() }
	fn indexOf(&self, item: &Box<Morphic>) -> int {
		let mut ix = 0;
		for it in self.subMorphs.iter() {
			if *it == *item { return ix; }
			ix += 1;
		}
		return -1;
	}
	fn contains(&self, item: &Box<Morphic>) -> bool { self.subMorphs.contains(item) }
//	//fn find(&self, predicate: TPredicate) -> &Morphic { self.subMorphs. }
	fn add(&mut self, item: Box<Morphic>) {
		//self.subMorphs.push(item);
	}
	fn remove(&mut self, item: Box<Morphic>) -> Box<Morphic> {
		let ix: int = self.indexOf(&item);
		assert!(ix != -1);
		self.subMorphs.remove(ix as uint).unwrap()
	}
//	fn each(&self, closure: TFunc) { self.subMorphs.each(closure) }
	//fn onItemsDo(&self, closure: TFunc) { self.subMorphs. }
	//fn allItemsDo(&self, closure: TFunc) { self.subMorphs. }
	//fn reverseAllItemsDo(&self, closure: TFunc) { self.subMorphs. }

	// Composite
    fn addMorph(&mut self, morph: Box<Morphic>) {
    	morph.removeFromOwner();
	    self.add(morph);
	    //self.damaged(&self.bounds());
//	    morph.onAdd(&Some(box *self as Box<Morphic>));
    }
    fn removeMorph(&mut self, morph: Box<Morphic>) -> Box<Morphic> {
	    let m = self.remove(morph);
	    //self.getTopParent().damaged(&self.bounds());
	    m
    }
    fn getParent<'s>(&'s self) -> &'s Option<Box<Morphic>> {
	    return &self.owner;
    }
    fn getTopParent(&self) -> &Box<Morphic> {
	    match self.owner {
	    	Some(ref owner) => return owner.getTopParent(),
	    	None => return  *self as Box<Morphic>
	    }
    }
    fn removeFromOwner(&self) {
	    match self.owner {
	    	Some(ref oldOwner) => {
//	    		oldOwner.removeMorph(box *self);
//	    		self.onAdd(&None);
	    	},
	    	None => {}
	    }
    }
}
/////////////////////////////////////////////
