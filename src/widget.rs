
use gfx::{Rect};
use nanovg::{Ctx};

enum Event {
	Key,
	Mouse
}
enum EventResult {
	Ignore,
	Consume
}

trait Widget {
	fn getParent(&self) -> Option<Self>;
	fn getTopParent(&self) -> Self;
	fn getBounds(&self) -> Rect;
	fn handleEvent(&self, evt: &Event) -> EventResult;
	fn damage(&self, rect: &Rect);
	fn update(&self);
	fn render(&self, ctx: &Ctx);
}