//! A 'property' is basically something that exposes both a getter and a setter.
//! Keep in mind that if you just need only either of those, a 'property'
//! wouldn't really be useful in the first place. Also keep in mind that this
//! implementation of properties only support immutable setters. This design is
//! suitable for our application because _Browser Window_ only exposes getters
//! and setters that call C functions that do all the work. There is no memory
//! unsafety caused by any of this.
//!
//! # Usage
//! ```ignore
//! use std::cell::Cell;
//! use std::ffi::OsString;
//!
//! struct MyStruct {
//!     oss: Cell<OsString>
//! }
//!
//! prop!{
//!     /// Your doc comments go here...
//!     pub MyProperty<String, &str>( this: MyStruct ) {
//!         get => this.oss.get().into_string().unwrap(),
//!         set(val) => this.oss.set( val.into() )
//!     }
//! }
//! ```
//! This property is called `MyProperty`, of which the getter returns a `String`
//! and the setter takes a `&str`. Also, the property will be part of
//! `MyStruct`, taking a reference to it in its `get` and `set` implementations
//! called `this` (note that the keyword `self` is taken already and can not be
//! used within the macro). The syntax was chosen to be somewhat Rust-like, but
//! require as little as possible boilerplate.
//!
//! There is one last thing that needs to be done, and that is that the property
//! needs to be added in the implementation of `MyStruct`: ```ignore
//! impl MyStruct {
//!     impl_prop!( pub my_property: MyProperty );
//! }
//! ```
//! Keep in mind that the `pub` keywords in both places are options.
//!
//! Then we have it.
//! The property now can be accessed like this:
//! ```ignore
//! let my_struct = MyStruct { Cell::new( OsString::new() ) };
//! let string = my_struct.my_property().get();
//! string.push_str("something");
//! my_struct.my_property().set( string );
//! ```

/// A property is something that has a setter and a getter.
// The setters are immutable.
// This is because they can not be changed from threads other than the GUI
// thread anyway.
pub trait Property<G, S> {
	fn get(&self) -> G;
	fn set(&self, value: S);
}

#[doc(hidden)]
#[macro_export]
macro_rules! _prop_internal {
	( $(#[$meta:meta])*, $vis:tt, $name:ident, $tg:ty, $ts:ty, $this:ident, $stype:ty, $get:expr, $val:ident, $set:expr ) => {

		// The struct is basically empty.
		$(#[$meta])*
		$vis struct $name<'a> {
			parent: &'a $stype
		}

		// And it implements the `Property` trait.
		impl<'a> Property<$tg,$ts> for $name<'a> {
			fn get( &self ) -> $tg { let $this = &self.parent; $get }

			fn set( &self, $val: $ts ) { let $this = &self.parent; $set }
		}
	}
}

/// A macro to define a so called 'property'.
/// Kind of similar to how C# properties work.
#[doc(hidden)]
#[macro_export]
macro_rules! prop {
	( $(#[$metas:meta])* $name:ident<$type:ty>( $this:ident: $stype:ty ) { get => $get:expr, set( $val:ident ) => $set:expr } ) => {
		 _prop_internal!( $(#[$metas])*, pub, $name, $type, $type, $this, $stype, $get, $val, $set  );
	};
	/*( $(#[$metas:meta])* pub $name:ident<$type:ty>( $this:ident: $stype:ty ) { get => $get:expr, set( $val:ident ) => $set:expr } ) => {
		 _prop_internal!( $(#[metas])*, pub, $name, $type, $type, $this, $stype, $get, $val, $set  );
	};
	( $(#[$metas:meta])* $name:ident<$tg:ty, $ts:ty>($this:ident: $stype:ty) { get => $get:expr, set( $val:ident ) => $set:expr } ) => {
		_prop_internal!( $(#[$metas])*, , $name, $tg, $ts, $this, $stype, $get, $val, $set  );
	};*/
	( $(#[$metas:meta])* pub $name:ident<$tg:ty, $ts:ty>($this:ident: $stype:ty) { get => $get:expr, set( $val:ident ) => $set:expr } ) => {
		_prop_internal!( $(#[$metas])*, pub, $name, $tg, $ts, $this, $stype, $get, $val, $set  );
	};
	( $(#[$metas:meta])* pub($vis:tt) $name:ident<$tg:ty, $ts:ty>($this:ident: $stype:ty) { get => $get:expr, set( $val:ident ) => $set:expr } ) => {
		_prop_internal!( $(#[$metas])*, pub($vis:tt), $name, $tg, $ts, $this, $stype, $get, $val, $set  );
	};
}

/// A macro to implement the property for a struct.
#[macro_export]
#[doc(hidden)]
macro_rules! impl_prop {
	($name:ident : $property:ident) => {
		fn $name<'a>(&'a self) -> $property {
			$property { parent: self }
		}
	};
	(pub $name:ident : $property:ident) => {
		pub fn $name<'a>(&'a self) -> $property {
			$property { parent: self }
		}
	};
}
