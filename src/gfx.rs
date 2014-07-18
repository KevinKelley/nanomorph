
//use nanovg;
//use std::cmp::{min,max}; // waah!  f32 isn't comparable!
pub use Color = nanovg::Color;

pub fn min(a:f32, b:f32) -> f32 { if a<b { a } else { b } }
pub fn max(a:f32, b:f32) -> f32 { if b<a { a } else { b } }

#[deriving(PartialEq, Clone, Show)]
pub struct Point {
	pub x: f32,
	pub y: f32
}

#[deriving(PartialEq, Clone, Show)]
pub struct Rect {
	x: f32,
	y: f32,
	w: f32,
	h: f32
}
impl Rect {
	pub fn zero() -> Rect { Rect { x:0.0, y:0.0, w:0.0, h:0.0 } }

	pub fn x(&self) -> f32 { self.x }
	pub fn y(&self) -> f32 { self.y }
	pub fn w(&self) -> f32 { self.w }
	pub fn h(&self) -> f32 { self.h }

	pub fn origin(&self) -> Point { Point { x: self.x(), y: self.y() } }
	pub fn corner(&self) -> Point { Point { x: self.x()+self.w(), y: self.y()+self.h() } }


	pub fn moveTo(&self, x: f32, y: f32) {

	}
	pub fn union(&self, other: &Rect) -> Rect {
		let x1 = min(self.x, other.x);
		let y1 = min(self.y, other.y);
		let x2 = max(self.x+self.w, other.x+other.w);
		let y2 = max(self.y+self.h, other.y+other.h);
		if x2 <= x1 || y2 <= y1 { return Rect::zero(); }
		Rect { x:x1, y:y1, w:x2-x1, h:y2-y1 }
	}
	pub fn intersection(&self, other: &Rect) -> Rect {
		let x1 = max(self.x, other.x);
		let y1 = max(self.y, other.y);
		let x2 = min(self.x+self.w, other.x+other.w);
		let y2 = min(self.y+self.h, other.y+other.h);
		if x2 < x1 || y2 < y1 { return Rect::zero(); }
		Rect { x:x1, y:y1, w:x2-x1, h:y2-y1 }
	}
    pub fn collides(&self, bbox: &Rect) -> bool {
	  // true iff self and bbox collide (intersect)
	  return  (bbox.x > self.x - bbox.w) &&
	          (bbox.y > self.y - bbox.h) &&
	          (bbox.x < self.x + self.w) &&
	          (bbox.y < self.y + self.h)
    }
    pub fn containsPoint(&self, x: f32, y: f32) -> bool {
	  // true iff abox contains point(x,y)
	  return  (x > self.x) &&
	          (y > self.y) &&
	          (x < self.x + self.w) &&
	          (y < self.y + self.h)
    }
    pub fn contains(&self, bbox: &Rect) -> bool {
	  // true if self contains bbox: that is, bbox is fully inside self
	  return  bbox.x >= self.x &&
	          bbox.y >= self.y &&
	          (bbox.x + bbox.w) <= (self.x + self.w) &&
	          (bbox.y + bbox.h) <= (self.y + self.h)
    }
}


//#[deriving(Eq, PartialEq, Hash, Clone, Show)]
//pub struct Color {
//	r: u8, g: u8, b: u8, a: u8
//}
//impl Color {
//	pub fn black() -> Color { Color{r:0,g:0,b:0,a:0} }
//
//	pub fn r(&self) -> f32 { self.r as f32 / 255.0 }
//	pub fn g(&self) -> f32 { self.g as f32 / 255.0 }
//	pub fn b(&self) -> f32 { self.b as f32 / 255.0 }
//	pub fn a(&self) -> f32 { self.a as f32 / 255.0 }
//}

//pub trait Brush {}

//type Graphics
//  #brush: Brush
//  #pen: Pen
//  #clip: Rect

pub trait Gfx {
	fn drawText(&self, txt: &str, x:f32, y:f32, c: Color);
	fn drawRect(&self, x:f32,y:f32, w:f32,h:f32, c: Color);
	fn fillRect(&self, x:f32,y:f32, w:f32,h:f32, c: Color);
	fn drawRoundRect(&self, x:f32,y:f32, w:f32,h:f32, cornerW:f32, cornerH:f32, c: Color);
	fn fillRoundRect(&self, x:f32,y:f32, w:f32,h:f32, cornerW:f32, cornerH:f32, c: Color);
	fn drawOval(&self, x:f32,y:f32, w:f32,h:f32, c: Color);
	fn fillOval(&self, x:f32,y:f32, w:f32,h:f32, c: Color);
	fn drawPolygon(&self, points: Vec<Point>);
}









