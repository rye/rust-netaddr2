use crate::Netv4Addr;
use crate::Netv6Addr;
use crate::{Error, Result};
use std::net::IpAddr;

/// A structure representing an IP network.
///
/// Internally using the built-in `std::net::IpAddr` structures, this is a
/// simple data structure that can be used in a variety of situations.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum NetAddr {
	/// An IPv4 network.
	V4(Netv4Addr),
	/// An IPv6 network.
	V6(Netv6Addr),
}

impl NetAddr {
	/// Get the "netmask" part of the inner `Netv4Addr` or the `Netv6Addr`.
	pub fn mask(&self) -> IpAddr {
		match self {
			Self::V4(v4) => IpAddr::V4(*v4.mask()),
			Self::V6(v6) => IpAddr::V6(*v6.mask()),
		}
	}

	/// Get the "network" part of the inner `Netv4Addr` or the `Netv6Addr`.
	pub fn addr(&self) -> IpAddr {
		match self {
			Self::V4(v4) => IpAddr::V4(*v4.addr()),
			Self::V6(v6) => IpAddr::V6(*v6.addr()),
		}
	}
}

mod broadcast;
mod contains;
mod display;
mod from;
mod fromstr;
mod hash;
mod merge;
mod partialord;

#[cfg(feature = "serde")]
mod de;
#[cfg(feature = "serde")]
mod ser;
