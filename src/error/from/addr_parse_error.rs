use std::net::AddrParseError;

use crate::error::Error;

impl From<AddrParseError> for Error {
	fn from(other: AddrParseError) -> Self {
		Self::ParseError(other.to_string())
	}
}
