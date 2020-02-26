use crate::traits::Mask;
use std::net::Ipv6Addr;

/// A structure representing an IPv6 network.
///
/// Internally, this structure includes two values; an `Ipv6Addr`
/// representing the network address (`addr`), and another
/// representing the netmask (`mask`).
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Ord)]
pub struct Netv6Addr {
	mask: Ipv6Addr,
	addr: Ipv6Addr,
}

impl Netv6Addr {
	pub const fn mask(&self) -> &Ipv6Addr {
		&self.mask
	}

	pub const fn addr(&self) -> &Ipv6Addr {
		&self.addr
	}

	pub fn is_cidr(&self) -> bool {
		let mask: u128 = self.mask.into();
		let ones: u32 = mask.count_ones();
		let cidr_mask: u128 = u128::max_value().checked_shl(128 - ones).unwrap_or(0);
		mask == cidr_mask
	}

	/// Create a new `Netv6Addr` from the given `addr` and `mask`.
	///
	/// Masks the given `addr` value with the given `mask` before
	/// the structure containing both is returned.
	///
	/// # Examples
	///
	/// ```rust
	/// # use netaddr2::Netv6Addr;
	/// # use std::net::Ipv6Addr;
	/// let network = Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 0x42);
	/// let netmask = Ipv6Addr::new(0xff00, 0, 0, 0, 0, 0, 0, 0);
	/// let netaddr = Netv6Addr::new(network, netmask);
	/// ```
	pub fn new(addr: Ipv6Addr, mask: Ipv6Addr) -> Self {
		let addr = addr.mask(&mask);
		Self { addr, mask }
	}
}

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
	use super::*;

	mod mask {
		use super::*;

		#[test]
		fn returns_mask_field() {
			let netaddr: Netv6Addr = Netv6Addr {
				mask: "ffff:ffff:ffff:ffff::0".parse().unwrap(),
				addr: "2001:db8:dead:beef::0".parse().unwrap(),
			};

			assert_eq!(
				netaddr.mask(),
				&"ffff:ffff:ffff:ffff::0".parse::<Ipv6Addr>().unwrap()
			);
		}
	}

	mod addr {
		use super::*;

		#[test]
		fn returns_addr_field() {
			let netaddr: Netv6Addr = Netv6Addr {
				mask: "ffff:ffff:ffff:ffff::0".parse().unwrap(),
				addr: "2001:db8:dead:beef::0".parse().unwrap(),
			};

			assert_eq!(
				netaddr.addr(),
				&"2001:db8:dead:beef::0".parse::<Ipv6Addr>().unwrap()
			);
		}
	}

	mod is_cidr {
		use super::*;

		#[test]
		fn non_cidr_returns_false() {
			let netaddr: Netv6Addr = Netv6Addr {
				mask: "ffff:ffff:ffff:7f7f::0".parse().unwrap(),
				addr: "::".parse().unwrap(),
			};

			assert_eq!(netaddr.is_cidr(), false);
		}

		#[test]
		fn cidr_returns_true() {
			let netaddr: Netv6Addr = Netv6Addr {
				mask: "ffff:ffff:ffff:fffc::0".parse().unwrap(),
				addr: "::".parse().unwrap(),
			};

			assert_eq!(netaddr.is_cidr(), true);
		}
	}

	mod new {
		use super::*;

		#[test]
		fn masks_addr() {
			let addr: Ipv6Addr = "2001:db8:dead:beef::0".parse().unwrap();
			let mask: Ipv6Addr = "ffff:ffff:ffff:ff00::0".parse().unwrap();

			let netaddr: Netv6Addr = Netv6Addr::new(addr, mask);

			assert_eq!(netaddr.mask(), &mask);
			assert_eq!(
				netaddr.addr(),
				&"2001:db8:dead:be00::0".parse::<Ipv6Addr>().unwrap()
			);
		}
	}
}
