#![crate_id = "nanomorph#0.1"]
#![desc = "a tiny morphic implementation on nanoVG"]
#![license = "MIT"]
#![crate_type = "lib"]

#![feature(globs)]
#![allow(non_snake_case_functions)]
#![allow(dead_code)]
#![allow(unused_variable)]

pub use gfx::*;
pub mod gfx;

type TFunc = fn (&Morphic) -> &Morphic;
type TPredicate = fn (&Morphic) -> bool;

trait Collection {
	fn numItems(&self) -> uint;
	fn indexOf(&self, item: &Box<Morphic>) -> int;
	fn contains(&self, item: &Box<Morphic>) -> bool;
	//fn find(&self, predicate: TPredicate) -> &Morphic;
	fn add(&self, item: &Box<Morphic>);
	fn remove(&self, item: &Box<Morphic>);
	//fn each(&self, closure: TFunc);
	//fn onItemsDo(&self, closure: TFunc);
	//fn allItemsDo(&self, closure: TFunc);
	//fn reverseAllItemsDo(&self, closure: TFunc);
}

trait Composite {
    fn addMorph(&self, morph: &Box<Morphic>);
    fn removeMorph(&self, morph: &Box<Morphic>);
    fn getParent<'s>(&'s self) -> &'s Option<Box<Morphic>>;
    fn getTopParent(&self) -> Box<Morphic>;
    fn removeFromOwner(&self);
}
trait BoundingBox {
    fn bounds(&self) -> Rect;
    fn inside(&self, x: f32, y: f32) -> bool;
    fn contains(&self, bbox: &Rect) -> bool;
    fn damaged(&self, r: &Rect);
}
trait Position {
    fn x(&self) -> f32;
    fn y(&self) -> f32;
    fn w(&self) -> f32;
    fn h(&self) -> f32;
    fn relativeX(&self) -> f32;
    fn relativeY(&self) -> f32;
    fn setPosition(&self, newX: f32, newY: f32);
    fn move(&self, dx: f32, dy: f32);
}
trait Size {
    fn setSize(&self, newWidth: f32, newHeight: f32);
    fn resize(&self, deltaW: f32, deltaH: f32);
    fn scale(&self, factor: f32);
}
trait Picking {
	// test for submorphs that fall outside this morph's bounds
    //fn testBadBounds(&self) -> bool;
	fn pick(&self, x: f32, y: f32) -> Vec<&Morphic>;
    fn dropTest(&self, morph: &Morphic) -> bool;
}
trait Colorable {
    fn getColor(&self) -> Color;
    fn setColor(&self, newColor: Color);
}

trait Morphic: Collection + Composite + BoundingBox {//+ Position + Size + Picking + Colorable {

}

#[deriving(PartialEq)] //Eq Clone Show Hash PartialEq
struct Morph {
    owner: Option<Box<Morphic>>,
    subMorphs: Vec<Box<Morphic>>,
    bounds: Rect,
    myColor: Color,
    pickMe: bool,
    pickChildren: bool
}
impl PartialEq for Box<Morphic> {
    fn eq(&self, other: &Box<Morphic>) -> bool { *self == *other }
}

impl Morphic for Morph {}

impl Collection for Morph {
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
	fn add(&self, item: &Box<Morphic>) {
		//self.subMorphs.push(item);
	}
	fn remove(&self, item: &Box<Morphic>) {
		//self.subMorphs.remove(item);
	}
//	fn each(&self, closure: TFunc) { self.subMorphs.each(closure) }
	//fn onItemsDo(&self, closure: TFunc) { self.subMorphs. }
	//fn allItemsDo(&self, closure: TFunc) { self.subMorphs. }
	//fn reverseAllItemsDo(&self, closure: TFunc) { self.subMorphs. }
}

impl Composite for Morph {
    fn addMorph(&self, morph: &Box<Morphic>) {
	    let p: &Option<Box<Morphic>> = morph.getParent();
	    let ref q = *p;
	    match q {
	    	Some(oldOwner) => oldOwner.removeMorph(morph),
	    	None => {}
	    }
	    self.add(morph);
	    self.damaged(&self.bounds());
//	    morph.onAdd(self);
    }
    fn removeMorph(&self, morph: &Box<Morphic>) {
	    self.remove(morph);
	    self.getTopParent().damaged(&self.bounds());
    }
    fn getParent<'s>(&'s self) -> &'s Option<Box<Morphic>> {
	    return &self.owner;
    }
    fn getTopParent(&self) -> Box<Morphic> {
	    // assumes parent is compositeMorph too,
	    // usually that's a given
	    match self.owner {
	    	Some(ref owner) => return owner.getTopParent(),
	    	None => return box *self as Box<Morphic>
	    }
    }
    fn removeFromOwner(&self) {
	    match self.owner {
	        Some(ref owner) => {
//	        	owner.removeMorph(self);
//	        	self.onAdd(None);
	        },
	        None => {}
	    }
    }
}

impl BoundingBox for Morph {
    fn bounds(&self) -> Rect { self.bounds }
    fn inside(&self, x: f32, y: f32) -> bool { self.bounds.containsPoint(x,y) }
    fn contains(&self, bbox: &Rect) -> bool { self.bounds.contains(bbox) }
    fn damaged(&self, r: &Rect) {  }
}

impl Position for Morph {
    fn x(&self) -> f32 { self.bounds().x() }
    fn y(&self) -> f32 { self.bounds().y() }
    fn w(&self) -> f32 { self.bounds().w() }
    fn h(&self) -> f32 { self.bounds().h() }
    fn relativeX(&self) -> f32 {
    	match self.owner {
	    	Some(ref owner) => return self.bounds().x() - owner.bounds().x ,
	    	None => return self.bounds().x()
    	}
    }
    fn relativeY(&self) -> f32 {
    	match self.owner {
	    	Some(ref owner) => return self.bounds().y() - owner.bounds().y ,
	    	None => return self.bounds().y()
    	}
    }
    fn setPosition(&self, newX: f32, newY: f32) {}
    fn move(&self, dx: f32, dy: f32) {}
}

impl Size for Morph {
    fn setSize(&self, newWidth: f32, newHeight: f32) {}
    fn resize(&self, deltaW: f32, deltaH: f32) {}
    fn scale(&self, factor: f32) {}
}

impl Picking for Morph {
	// test for submorphs that fall outside this morph's bounds
    //fn testBadBounds(&self) -> bool;
	fn pick(&self, x: f32, y: f32) -> Vec<&Morphic> { vec!() }
    fn dropTest(&self, morph: &Morphic) -> bool { false }
}

impl Colorable for Morph {
    fn getColor(&self) -> Color { Color::black() }
    fn setColor(&self, newColor: Color) {}
}
