


#[cfg(not(feature = "threadsafe"))]
pub(crate) type Rc<T> = std::rc::Rc<T>;
#[cfg(feature = "threadsafe")]
pub(crate) type Rc<T> = std::sync::Arc<T>;