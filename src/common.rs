pub trait HasHandle<H> {
	fn handle(&self) -> &H;
}


impl<H> HasHandle<H> for H {
	fn handle(&self) -> &H { self }
}