use core::fmt::{Display, Error, Formatter};
use std::net::AddrParseError;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NetAddrError {
	ParseError(String),
}

impl Display for NetAddrError {
	fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
		match self {
			Self::ParseError(text) => write!(f, "unable to parse address: {}", text),
		}
	}
}

impl From<AddrParseError> for NetAddrError {
	fn from(other: AddrParseError) -> Self {
		Self::ParseError(other.to_string())
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	mod display {
		use super::*;

		#[test]
		fn right_message() {
			let error: NetAddrError = NetAddrError::ParseError("INNER_TEXT".into());
			let result: &str = &format!("{}", error);
			assert_eq!(result, "unable to parse address: INNER_TEXT");
		}
	}
}
