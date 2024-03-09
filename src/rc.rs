#[cfg(not(feature = "threadsafe"))]
pub(crate) type Weak<T> = std::rc::Weak<T>;
#[cfg(feature = "threadsafe")]
pub(crate) type Weak<T> = std::sync::Weak<T>;
#[cfg(not(feature = "threadsafe"))]
pub(crate) type Rc<T> = std::rc::Rc<T>;
#[cfg(feature = "threadsafe")]
pub(crate) type Rc<T> = std::sync::Arc<T>;
