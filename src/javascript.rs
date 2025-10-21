use std::{borrow::Cow, collections::HashMap, fmt, str::FromStr};

use json::JsonValue;
pub use num_bigfloat::BigFloat;

/// A JavaScript value.
#[derive(Clone, Debug)]
pub enum JsValue {
	Array(Vec<JsValue>),
	Boolean(bool),
	Null,
	Number(BigFloat),
	Object(HashMap<String, JsValue>),
	String(String),
	Undefined,
	/// When a javascript value is returned that does not fit any of the other
	/// value types in this enum, `JsValue::Other` is returned with a string
	/// representation of the value. For example, functions get returned as an
	/// instance of `JsValue::Other`.
	Other(String),
}

impl JsValue {
	/// Parses the given JSON string into a `JsValue`. If parsing failed, the
	/// string is returned as a `JsValue::Other`.
	pub fn from_json(string: &str) -> Self {
		match json::parse(string) {
			Err(_) => JsValue::Other(string.to_string()),
			Ok(value) => Self::_from_json(value),
		}
	}

	pub fn from_string(string: &str) -> Self {
		if string.len() == 0 {
			return Self::Other(String::new());
		}

		// If the symbol starts with a digit, interpret it as a (positive) number
		if "0123456789".contains(|c| c == string.chars().nth(0).unwrap()) {
			return match BigFloat::from_str(string) {
				Err(_) => Self::Other(format!("unable to parse number: {}", string)),
				Ok(f) => Self::Number(f),
			};
		}
		if string == "null" {
			return Self::Null;
		}
		if string == "undefined" {
			return Self::Undefined;
		}
		if string == "true" {
			return Self::Boolean(true);
		}
		if string == "false" {
			return Self::Boolean(false);
		}
		if string.chars().nth(0) == Some('\"') && string.chars().last() == Some('\"') {
			return Self::String(string[1..(string.len() - 1)].to_string());
		}
		if string.chars().nth(0) == Some('[') && string.chars().last() == Some(']') {
			let mut array = Vec::new();
			for part in string[1..(string.len() - 1)].split(',') {
				array.push(Self::from_string(part));
			}
			return Self::Array(array);
		}
		if string.chars().nth(0) == Some('{') && string.chars().last() == Some('}') {
			let mut map = HashMap::new();
			for part in string[1..(string.len() - 1)].split(',') {
				if let Some((key, value)) = part.split_once(':') {
					map.insert(key.to_string(), Self::from_string(value));
				}
			}
			return Self::Object(map);
		}

		Self::Other(string.to_string())
	}

	fn _from_json(value: JsonValue) -> Self {
		match value {
			JsonValue::Null => Self::Null,
			JsonValue::Short(s) => Self::String(s.to_string()),
			JsonValue::String(s) => {
				println!("S '{}'", s);
				Self::String(s)
			}
			JsonValue::Number(n) => {
				let (sign, mantissa, exponent) = n.as_parts();

				let big: BigFloat = mantissa.into();
				for _i in 0..exponent {
					big.mul(&10.into());
				}
				if !sign {
					big.inv_sign();
				}
				Self::Number(big)
			}
			JsonValue::Boolean(b) => Self::Boolean(b),
			JsonValue::Object(o) => {
				let mut map = HashMap::with_capacity(o.len());
				for (key, value) in o.iter() {
					map.insert(key.to_string(), Self::_from_json(value.clone()));
				}
				Self::Object(map)
			}
			JsonValue::Array(a) =>
				Self::Array(a.into_iter().map(|i| Self::_from_json(i)).collect()),
		}
	}

	/// Gets the string of the `JsValue::String`, or otherwise just a normal
	/// string representation of the value.
	pub fn to_string_unenclosed(&self) -> Cow<'_, str> {
		match self {
			Self::String(s) => Cow::Borrowed(s),
			other => Cow::Owned(other.to_string()),
		}
	}
}

impl fmt::Display for JsValue {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Array(a) => {
				write!(f, "[")?;
				if a.len() > 0 {
					write!(f, "{}", a[0])?;
					for i in 1..a.len() {
						write!(f, ",{}", a[i])?;
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
			Self::String(s) => write!(f, "\"{}\"", escape_string(s)),
			Self::Undefined => write!(f, "undefined"),
			Self::Other(code) => write!(f, "{}", code),
		}
	}
}

const UNESCAPED_CHARACTERS: &str =
	" -_0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!#$%&()*+,./:;<=>?@[]^`{|}~";

fn escape_string(string: &str) -> Cow<'_, str> {
	if string.len() == 0 {
		return Cow::Borrowed(string);
	}

	let mut result = String::with_capacity(string.len() * 2);
	let mut escaped = 0;
	for char in string.chars() {
		// TODO: Clean up this code
		if !UNESCAPED_CHARACTERS.contains(char) {
			escaped += 1;
			if char == '\n' {
				result.push('\\');
				result.push('n');
			} else if char == '\'' {
				result.push('\\');
				result.push('\'');
			} else if char == '"' {
				result.push('\\');
				result.push('"');
			} else if char == '\r' {
				result.push('\\');
				result.push('r');
			} else if char == '\t' {
				result.push('\\');
				result.push('t');
			} else if char == '\r' {
				result.push('\\');
				result.push('r');
			} else if char == '\x08' {
				result.push('\\');
				result.push('b');
			} else if char == '\x0c' {
				result.push('\\');
				result.push('f');
			} else if char == '\x0b' {
				result.push('\\');
				result.push('v');
			} else if char == '\0' {
				result.push('\\');
				result.push('0');
			} else if (char as u32) < 256 {
				let codepoint = char as u32;
				result.push('\\');
				result.push('x');
				result.push(char::from_digit(codepoint >> 4, 16).unwrap());
				result.push(char::from_digit(codepoint & 0x0F, 16).unwrap());
			} else {
				result.extend(char.escape_unicode().into_iter());
			}
		} else {
			result.push(char);
		}
	}

	// Even though we've already created a new string, we still save memory by
	// dropping it:
	if escaped == 0 {
		Cow::Borrowed(string)
	} else {
		Cow::Owned(result)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_string_escaping() {
		let input = " test\r\n\t\x08\x0b\x0c\0\x7f\u{1234}❤©";
		let output = " test\\r\\n\\t\\b\\v\\f\\0\\x7f\\u{1234}\\u{2764}\\xa9";
		assert_eq!(output, escape_string(input))
	}
}
