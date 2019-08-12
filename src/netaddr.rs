use crate::netv4addr::Netv4Addr;
use crate::netv6addr::Netv6Addr;
use std::net::IpAddr;

/// A structure representing an IP network.
///
/// Internally using the built-in `std::net::IpAddr` structures, this is a
/// simple data structure that can be used in a variety of situations.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum NetAddr {
	V4(Netv4Addr),
	V6(Netv6Addr),
}

impl NetAddr {
	pub fn mask(&self) -> IpAddr {
		match self {
			NetAddr::V4(v4) => IpAddr::V4(*v4.mask()),
			NetAddr::V6(v6) => IpAddr::V6(*v6.mask()),
		}
	}

	pub fn addr(&self) -> IpAddr {
		match self {
			NetAddr::V4(v4) => IpAddr::V4(*v4.addr()),
			NetAddr::V6(v6) => IpAddr::V6(*v6.addr()),
		}
	}
}
