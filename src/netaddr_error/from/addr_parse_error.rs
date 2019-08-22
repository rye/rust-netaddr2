use super::NetAddrError;
use std::net::AddrParseError;

impl From<AddrParseError> for NetAddrError {
	fn from(other: AddrParseError) -> Self {
		Self::ParseError(other.to_string())
	}
}
