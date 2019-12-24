use super::Error;
use core::fmt::{self, Display, Formatter};

impl Display for Error {
	fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
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
		let error: Error = Error::ParseError("INNER_TEXT".into());
		let result: &str = &format!("{}", error);
		assert_eq!(result, "unable to parse address: INNER_TEXT");
	}
}
