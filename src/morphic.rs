#![crate_name = "nanomorph"]
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



type TFunc = fn (&Morphic) -> &Morphic;
type TPredicate = fn (&Morphic) -> bool;

trait Collection {
	fn numItems(&self) -> uint;
	fn indexOf(&self, item: &Morphic) -> int;
	fn contains(&self, item: &Morphic) -> bool;
	//fn find(&self, predicate: TPredicate) -> &Morphic;
	fn add(&mut self, item: &Morphic);
	fn remove(&mut self, item: &Morphic) -> &Morphic;
	//fn each(&self, closure: TFunc);
	// apply closure to each of my own children
	//fn onItemsDo(&self, closure: TFunc);
	// recursively apply closure to all my descendants, in pre-order
	//fn allItemsDo(&self, closure: TFunc);
	// recursively apply closure to all my descendants, in post-order
	//fn reverseAllItemsDo(&self, closure: TFunc);
}

trait Composite {
    fn addMorph(&mut self, morph: &Morphic);
    fn removeMorph(&mut self, morph: &Morphic) -> &Morphic;
    //fn getParent<'s>(&'s mut self) -> &'s Option<Morphic>;
    fn getParent(&mut self) -> Option<&mut Morphic>;
    //fn getTopParent(&mut self) -> Morphic;
    fn getTopParent(&mut self) -> &mut Morphic;
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
    fn relativeX(&self) -> f32;
    fn relativeY(&self) -> f32;
    fn setPosition(&self, newX: f32, newY: f32);
    fn move(&self, dx: f32, dy: f32);
}
trait Size {
    fn w(&self) -> f32;
    fn h(&self) -> f32;
    fn setSize(&mut self, newWidth: f32, newHeight: f32);
    //fn resize(&mut self, deltaW: f32, deltaH: f32);
    //fn scale(&mut self, factor: f32);
}
trait Picking {
	// test for submorphs that fall outside this morph's bounds
    //fn testBadBounds(&self) -> bool;
	fn pick(&self, x: f32, y: f32) -> Vec<&Morphic>;
    fn dropTest(&self, morph: &Morphic) -> bool;
}
trait Colorable {
    fn getColor(&self) -> Color;
    fn setColor(&mut self, newColor: Color);
}
trait Draw {
    fn draw(&self, gfx: &mut Gfx);
}


trait Morphic: Collection+Composite+BoundingBox+Position+Size+Picking+Colorable+Draw {
	fn onAdd(&self, newparent: &Option<&Morphic>);
}

#[deriving(PartialEq)] //Eq Clone Show Hash PartialEq
struct BaseMorph {
    owner: Rawlink<Box<Morphic>>,
    subMorphs: Vec<Box<Morphic>>,
    bounds: Rect,
    myColor: Color,
    pickMe: bool,
    pickChildren: bool
}
impl PartialEq for Box<Morphic> {
    fn eq(&self, other: &Box<Morphic>) -> bool { *self == *other }
}

impl Collection for BaseMorph {
	fn numItems(&self) -> uint { self.subMorphs.len() }
	fn indexOf(&self, item: &Morphic) -> int {
		let mut ix = 0;
		for it in self.subMorphs.iter() {
			if it == item { return ix; }
			ix += 1;
		}
		return -1;
	}
	fn contains(&self, item: &Morphic) -> bool { self.subMorphs.contains(item) }
//	//fn find(&self, predicate: TPredicate) -> &Morphic { self.subMorphs. }
	fn add(&mut self, item: &Morphic) {
		//self.subMorphs.push(item);
	}
	fn remove(&mut self, item: &Morphic) -> &Morphic {
		let ix: int = self.indexOf(item);
		assert!(ix != -1);
		self.subMorphs.remove(ix as uint).unwrap()
	}
//	fn each(&self, closure: TFunc) { self.subMorphs.each(closure) }
	//fn onItemsDo(&self, closure: TFunc) { self.subMorphs. }
	//fn allItemsDo(&self, closure: TFunc) { self.subMorphs. }
	//fn reverseAllItemsDo(&self, closure: TFunc) { self.subMorphs. }
}

impl Composite for BaseMorph {
    fn addMorph(&mut self, morph: &Morphic) {
    	morph.removeFromOwner();
	    self.add(morph);
	    self.damaged(&self.bounds());
//	    morph.onAdd(&Some(box *self as Morphic));
    }
    fn removeMorph(&mut self, morph: &Morphic) -> &Morphic {
	    let m = self.remove(morph);
	    self.getTopParent().damaged(&self.bounds());
	    m
    }

	//fn getParent(&mut self) -> Option<&mut TreeNode> { self.parent.resolve() }
    fn getParent(&mut self) -> Option<&mut Morphic> { self.owner.resolve() }

	//fn getRoot<'a>(&'a mut self) -> &'a mut TreeNode {
	//	match self.getParent() {
	//		Some(p) => p.getRoot(),
	//		None => self
	//	}
	//}
    //fn getTopParent<'a>(&'a mut self) -> &'a mut Morphic {
    fn getTopParent(&mut self) -> &mut Morphic {
		match self.getParent() {
			Some(p) => p.getTopParent(),
			None => self
		}
    }
//+    fn getTopParent(&self) -> &Morphic {
//+	    match self.owner {
//+	    	Some(ref owner) => return owner.getTopParent(),
//+	    	None => return  *self as Morphic
//+	    }
//+    }
    fn removeFromOwner(&self) {
	    match self.getParent() {
	    	Some(ref oldOwner) => {
//	    		oldOwner.removeMorph(box *self);
	    		self.onAdd(&None);
	    	},
	    	None => {}
	    }
    }
}

impl BoundingBox for BaseMorph {
    fn bounds(&self) -> Rect { self.bounds }
    fn inside(&self, x: f32, y: f32) -> bool { self.bounds.containsPoint(x,y) }
    fn contains(&self, bbox: &Rect) -> bool { self.bounds.contains(bbox) }
    fn damaged(&self, r: &Rect) {  }
}

impl Position for BaseMorph {
    fn x(&self) -> f32 { self.bounds().x() }
    fn y(&self) -> f32 { self.bounds().y() }
    fn relativeX(&self) -> f32 {
    	match self.getParent() {
	    	Some(ref owner) => return self.bounds().x() - owner.bounds().x ,
	    	None => return self.bounds().x()
    	}
    }
    fn relativeY(&self) -> f32 {
    	match self.getParent() {
	    	Some(ref owner) => return self.bounds().y() - owner.bounds().y ,
	    	None => return self.bounds().y()
    	}
    }
    fn setPosition(&self, newX: f32, newY: f32) {}
    fn move(&self, dx: f32, dy: f32) {}
}

impl Size for BaseMorph {
    fn w(&self) -> f32 { self.bounds().w() }
    fn h(&self) -> f32 { self.bounds().h() }
    fn setSize(&mut self, newWidth: f32, newHeight: f32) {}
    //fn resize(&mut self, deltaW: f32, deltaH: f32) {}
    //fn scale(&mut self, factor: f32) {}
}

impl Picking for BaseMorph {
	// test for submorphs that fall outside this morph's bounds
    //fn testBadBounds(&self) -> bool;
	fn pick(&self, x: f32, y: f32) -> Vec<&Morphic> { vec!() }
    fn dropTest(&self, morph: &Morphic) -> bool { false }
}

impl Colorable for BaseMorph {
    fn getColor(&self) -> Color { Color::black() }
    fn setColor(&mut self, newColor: Color) {}
}

impl Draw for BaseMorph {
    fn draw(&self, gfx: &mut Gfx) {}
}

impl Morphic for BaseMorph {
	fn onAdd(&self, newparent: &Option<&Morphic>) {}
}
