use super::NetAddrError;
use core::fmt::{Display, Error, Formatter};

impl Display for NetAddrError {
	fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
		match self {
			Self::ParseError(text) => write!(f, "unable to parse address: {}", text),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn right_message() {
		let error: NetAddrError = NetAddrError::ParseError("INNER_TEXT".into());
		let result: &str = &format!("{}", error);
		assert_eq!(result, "unable to parse address: INNER_TEXT");
	}
}
