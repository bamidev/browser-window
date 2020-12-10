use std::{
	boxed::Box,
	future::Future,
	pin::Pin
};



pub struct Event<'a,A> {
	handlers: Vec<Box<dyn FnMut( &A ) -> Pin<Box<dyn Future<Output=()> + 'a>> + 'a>>
}



impl<'a,A> Event<'a,A> {

	/// Invokes the event, which calls all handlers that have been registered to this event.
	pub(in crate) fn invoke( &mut self, args: &A  ) {

		for h in self.handlers.iter_mut() {
			h( args );
		}
	}

	/// Register a closure to be invoked for this event.
	pub fn register<H>( &mut self, mut handler: H ) where
		H: FnMut( &A ) + 'a
	{
		self.handlers.push(Box::new(move |args| {
			handler( args );
			Box::pin(async {})
		}));
	}

	/// Register an 'async closre' to be invoked for this event.
	///
	/// # Example
	/// ```rust
	/// my_event.register_async(|args| async move {
	///     // Do something ...
	/// });
	/// ```
	pub fn register_async<H,F>( &mut self, mut handler: H ) where
		H: FnMut( &A ) -> F + 'a,
		F: Future<Output=()> + 'a
	{
		self.handlers.push(Box::new(move |args| Box::pin( handler( args ) ) ) );
	}
}

impl<'a,A> Default for Event<'a,A> {
	
	fn default() -> Self {
		Self {
			handlers: Vec::new()
		}
	}
}