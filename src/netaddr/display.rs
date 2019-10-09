use super::NetAddr;
use core::fmt;

impl fmt::Display for NetAddr {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::V4(addr) => write!(f, "{}", addr),
			Self::V6(addr) => write!(f, "{}", addr),
		}
	}
}
