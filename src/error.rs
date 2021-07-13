/// The error type for operations relating to the `NetAddr` type
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum Error {
	ParseError(String),
}

mod display;
mod from;

mod result;
pub use result::*;

#[cfg(test)]
mod tests {
	#[test]
	fn parse_error_construction() {
		let _error: crate::Error = crate::Error::ParseError("yote".to_string());
	}
}
