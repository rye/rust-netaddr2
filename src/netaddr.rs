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

	/// Return whether the inner `Netv4Addr` or `Netv6Addr` is CIDR.
	///
	/// A `Netv4Addr` or `Netv6Addr` is CIDR if and only if its underlying
	/// netmask is "left contigous"; that is, if its bit pattern is a given
	/// number of ones followed by a remaining group of zeroes.
	pub fn is_cidr(&self) -> bool {
		match self {
			Self::V4(v4) => v4.is_cidr(),
			Self::V6(v6) => v6.is_cidr(),
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

#[cfg(test)]
mod tests {
	use super::NetAddr;

	mod is_cidr {
		use super::*;

		mod v4 {
			use super::*;

			#[test]
			fn non_cidr_returns_false() {
				let netaddr: NetAddr = "0.0.0.0/255.255.127.0".parse().unwrap();
				assert_eq!(netaddr.is_cidr(), false);
			}

			#[test]
			fn cidr_returns_true() {
				let netaddr: NetAddr = "0.0.0.0/255.255.192.0".parse().unwrap();
				assert_eq!(netaddr.is_cidr(), true);
			}
		}

		mod v6 {
			use super::*;

			#[test]
			fn non_cidr_returns_false() {
				let netaddr: NetAddr = "::/ffff:ffff:fff::".parse().unwrap();
				assert_eq!(netaddr.is_cidr(), false);
			}

			#[test]
			fn cidr_returns_true() {
				let netaddr: NetAddr = "::/ffff:ffff:fffc::".parse().unwrap();
				assert_eq!(netaddr.is_cidr(), true);
			}
		}
	}
}
