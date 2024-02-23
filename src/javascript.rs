use std::{borrow::Cow, collections::HashMap, fmt};

pub use num_bigfloat::BigFloat;

/// A JavaScript value.
pub enum JsValue {
	Array(Vec<JsValue>),
	Boolean(bool),
	Null,
	Number(BigFloat),
	Object(HashMap<String, JsValue>),
	String(String),
	Undefined,
	/// When a javascript value is returned that does not fit any of the other value types in this enum, `JsValue::Other` is returned with a string representation of the value.
	/// When using feature `cef`, it currently always returns the javascript value as `JsValue::Other`.
	Other(String),
}

impl fmt::Display for JsValue {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Array(a) => {
				write!(f, "[")?;
				if a.len() > 0 {
					write!(f, "{}", a[0])?;
					for i in a {
						write!(f, "{}", i)?;
					}
				}
				write!(f, "]")
			}
			Self::Boolean(b) => write!(f, "{}", b),
			Self::Number(n) => write!(f, "{}", n),
			Self::Object(o) => {
				write!(f, "{{")?;
				for (k, v) in o.iter() {
					write!(f, "\"{}\":{}", k, v)?;
				}
				write!(f, "}}")
			}
			Self::Null => write!(f, "null"),
			Self::String(s) => write!(f, "'{}'", escape_string(s)),
			Self::Undefined => write!(f, "undefined"),
			Self::Other(code) => write!(f, "{}", code),
		}
	}
}

const UNESCAPED_CHARACTERS: &str =
	"_0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ@\\*_+-./";

fn escape_string(string: &str) -> Cow<'_, str> {
	if string.len() == 0 {
		return Cow::Borrowed(string);
	}

	let mut result = String::with_capacity(string.len() * 2);
	for char in string.chars() {
		if !UNESCAPED_CHARACTERS.contains(char) {
			result.push('\\');
		}
		result.push(char);
	}
	Cow::Owned(result)
}
