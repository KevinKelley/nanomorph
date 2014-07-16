

trait Widget {
	fn getParent(&self) -> Option<Self>;
	fn getTopParent(&self) -> Self;
	fn getBounds(&self) -> Rect;
	fn handleEvent(&self, evt: &Event) -> Consume;
	fn damage(&self, rect: &Rect);
	fn update(&self);
	fn render(&self, ctx: &Context);
}