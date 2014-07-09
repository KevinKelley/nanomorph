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


#[deriving(PartialEq)] //Eq Clone Show Hash PartialEq
struct Morphic {
    owner: Rawlink<Morphic>,
    subMorphs: Vec<Box<Morphic>>,
    bounds: Rect,
    myColor: Color,
    pickMe: bool,
    pickChildren: bool
}

impl Morphic
{
//impl Collection for BaseMorph {
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
	fn remove(&mut self, item: &Box<Morphic>) -> Box<Morphic> {
		let ix: int = self.indexOf(item);
		assert!(ix != -1);
		self.subMorphs.remove(ix as uint).unwrap()
	}
//	fn each(&self, closure: TFunc) { self.subMorphs.each(closure) }
	//fn onItemsDo(&self, closure: TFunc) { self.subMorphs. }
	//fn allItemsDo(&self, closure: TFunc) { self.subMorphs. }
	//fn reverseAllItemsDo(&self, closure: TFunc) { self.subMorphs. }

//impl Composite for BaseMorph {
    fn addMorph(&mut self, morph: Box<Morphic>) {
    	morph.removeFromOwner();
	    self.add(morph);
	    self.damaged(&self.bounds());
//	    morph.onAdd(&Some(box *self as Box<Morphic>));
    }
    fn removeMorph(&mut self, morph: &Box<Morphic>) -> Box<Morphic> {
	    let m = self.remove(morph);
        let bounds = self.bounds();
	    self.getTopParent().damaged(&bounds);
	    m
    }

    fn getParent<'a>(&'a  self) -> Option<&'a  Morphic> {
        self.owner.resolve_immut()
    }
    fn getTopParent<'a>(&'a  self) -> &'a  Morphic {
		match self.getParent() {
			Some(p) => p.getTopParent(),
			None => self
		}
    }
    fn removeFromOwner(&self) {
	    match self.getParent() {
	    	Some(ref oldOwner) => {
	    		oldOwner.removeMorph(self);
	    		self.onAdd(&None);
	    	},
	    	None => {}
	    }
    }

//impl BoundingBox for BaseMorph {
    fn bounds(&self) -> Rect { self.bounds }
    fn inside(&self, x: f32, y: f32) -> bool { self.bounds.containsPoint(x,y) }
    fn containsRect(&self, bbox: &Rect) -> bool { self.bounds.contains(bbox) }
    fn damaged(&self, r: &Rect) {  }

//impl Position for BaseMorph {
    fn x(&self) -> f32 { self.bounds().x() }
    fn y(&self) -> f32 { self.bounds().y() }
    fn w(&self) -> f32 { self.bounds().w() }
    fn h(&self) -> f32 { self.bounds().h() }
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

//impl Size for BaseMorph {
    fn setSize(&mut self, newWidth: f32, newHeight: f32) {}
    fn resize(&mut self, deltaW: f32, deltaH: f32) {}
    fn scale(&mut self, factor: f32) {}

//impl Picking for BaseMorph {
	// test for submorphs that fall outside this morph's bounds
    //fn testBadBounds(&self) -> bool;
	fn pick(&self, x: f32, y: f32) -> Vec<&Morphic> { vec!() }
    fn dropTest(&self, morph: &Morphic) -> bool { false }

//impl Colorable for BaseMorph {
    fn getColor(&self) -> Color { Color::black() }
    fn setColor(&mut self, newColor: Color) {}

//impl Morphic for BaseMorph {
	fn onAdd(&self, newparent: &Option<Box<Morphic>>) {}
}
