use super::Error;
use std::net::AddrParseError;

impl From<AddrParseError> for Error {
	fn from(other: AddrParseError) -> Self {
		Self::ParseError(other.to_string())
	}
}
