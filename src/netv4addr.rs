use crate::traits::Mask;

pub trait Ipv4Addr:
	Copy
	+ Clone
	+ core::fmt::Debug
	+ PartialEq
	+ Eq
	+ core::hash::Hash
	+ PartialOrd
	+ Ord
	+ Mask<Output = Self>
	+ Sized
	+ Into<u32>
{
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
struct OctetsIpv4Addr([u8; 4]);

impl From<OctetsIpv4Addr> for u32 {
	fn from(oia: OctetsIpv4Addr) -> Self {
		u32::from_le_bytes(oia.0)
	}
}

impl From<u32> for OctetsIpv4Addr {
	fn from(addr: u32) -> Self {
		Self(addr.to_le_bytes())
	}
}

impl From<[u8; 4]> for OctetsIpv4Addr {
	fn from(octets: [u8; 4]) -> Self {
		Self(octets)
	}
}

impl Mask for OctetsIpv4Addr {
	type Output = Self;
	fn mask(&self, other: &Self) -> Self {
		Self([
			self.0[0] & other.0[0],
			self.0[1] & other.0[1],
			self.0[2] & other.0[2],
			self.0[3] & other.0[3],
		])
	}
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct UintIpv4Addr(u32);

impl From<UintIpv4Addr> for u32 {
	fn from(uia: UintIpv4Addr) -> Self {
		uia.0
	}
}

impl From<u32> for UintIpv4Addr {
	fn from(addr: u32) -> Self {
		Self(addr)
	}
}

impl From<[u8; 4]> for UintIpv4Addr {
	fn from(octets: [u8; 4]) -> Self {
		Self(u32::from_le_bytes(octets))
	}
}

impl Mask for UintIpv4Addr {
	type Output = Self;
	fn mask(&self, other: &Self) -> Self {
		Self(self.0 & other.0)
	}
}

impl Ipv4Addr for std::net::Ipv4Addr {}
impl Ipv4Addr for OctetsIpv4Addr {}
impl Ipv4Addr for UintIpv4Addr {}

/// A structure representing an IPv4 network.
///
/// Internally, this structure includes two values; an `Ipv4Addr`
/// representing the network address (`addr`), and another
/// representing the netmask (`mask`).
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Netv4Addr<Addr: Ipv4Addr = std::net::Ipv4Addr> {
	addr: Addr,
	mask: Addr,
}

impl<Addr: Ipv4Addr + Sized> Netv4Addr<Addr> {
	#[inline]
	pub fn mask(&self) -> Addr {
		self.mask
	}

	#[inline]
	pub fn addr(&self) -> Addr {
		self.addr
	}

	#[allow(clippy::trivially_copy_pass_by_ref)]
	pub fn is_cidr(&self) -> bool {
		let mask: u32 = self.mask.into();
		let ones: u32 = mask.count_ones();
		let cidr_mask: u32 = u32::max_value().checked_shl(32 - ones).unwrap_or(0);
		mask == cidr_mask
	}

	/// Create a new `Netv4Addr` from the given `addr` and `mask`.
	///
	/// Masks the given `addr` value with the given `mask` before
	/// the structure containing both is returned.
	///
	/// # Examples
	///
	/// ```rust
	/// # use netaddr2::Netv4Addr;
	/// # use std::net::Ipv4Addr;
	/// let network = Ipv4Addr::new(127, 0, 1, 1);
	/// let netmask = Ipv4Addr::new(255, 0, 0, 0);
	/// let netaddr = Netv4Addr::new(network, netmask);
	///
	/// // We do need to have the `Contains` trait in scope to use it...
	/// use netaddr2::Contains;
	/// let test: Ipv4Addr = Ipv4Addr::new(127, 47, 23, 37);
	/// assert!(netaddr.contains(&test));
	/// ```
	pub fn new(addr: Addr, mask: Addr) -> Self {
		let addr = addr.mask(&mask);
		Self { addr, mask }
	}
}

mod broadcast;
mod contains;
mod display;
mod from;
mod fromstr;
mod hash;
mod merge;
mod ord;
mod partialord;

#[cfg(feature = "serde")]
mod de;
#[cfg(feature = "serde")]
mod ser;

#[cfg(test)]
mod tests {
	use super::Netv4Addr;
	use std::net::Ipv4Addr;

	mod mask {
		use super::*;

		#[test]
		fn returns_mask_field() {
			let netaddr: Netv4Addr = Netv4Addr {
				mask: "255.255.255.255".parse().unwrap(),
				addr: "0.0.0.0".parse().unwrap(),
			};

			assert_eq!(
				netaddr.mask(),
				"255.255.255.255".parse::<Ipv4Addr>().unwrap()
			);
		}
	}

	mod addr {
		use super::*;

		#[test]
		fn returns_addr_field() {
			let netaddr: Netv4Addr = Netv4Addr {
				mask: "255.255.255.255".parse().unwrap(),
				addr: "0.0.0.0".parse().unwrap(),
			};

			assert_eq!(netaddr.addr(), "0.0.0.0".parse::<Ipv4Addr>().unwrap());
		}
	}

	mod is_cidr {
		use super::*;

		#[test]
		fn non_cidr_returns_false() {
			let netaddr: Netv4Addr = Netv4Addr {
				mask: "255.127.255.0".parse().unwrap(),
				addr: "0.0.0.0".parse().unwrap(),
			};

			assert!(!netaddr.is_cidr());
		}

		#[test]
		fn cidr_returns_true() {
			let netaddr: Netv4Addr = Netv4Addr {
				mask: "255.224.0.0".parse().unwrap(),
				addr: "0.0.0.0".parse().unwrap(),
			};

			assert!(netaddr.is_cidr());
		}
	}

	mod new {
		use super::*;

		#[test]
		fn masks_addr() {
			let addr: Ipv4Addr = "192.168.16.32".parse().unwrap();
			let mask: Ipv4Addr = "255.64.128.3".parse().unwrap();

			let netaddr: Netv4Addr = Netv4Addr::new(addr, mask);

			assert_eq!(netaddr.mask(), mask);
			assert_eq!(netaddr.addr(), "192.0.0.0".parse::<Ipv4Addr>().unwrap());
		}
	}
}

#[cfg(test)]
mod octets_ipv4_addr {
	use super::{Netv4Addr, OctetsIpv4Addr};

	#[test]
	fn can_construct_netv4addr() {
		use crate::traits::Contains;
		let addr: [u8; 4] = [127, 0, 0, 1];
		let mask: [u8; 4] = [255, 0, 0, 0];
		let addr: OctetsIpv4Addr = addr.into();
		let mask: OctetsIpv4Addr = mask.into();
		let net: Netv4Addr<OctetsIpv4Addr> = Netv4Addr::new(addr, mask);

		let test: OctetsIpv4Addr = [127, 1, 2, 3].into();

		assert!(net.contains(&test));
	}
}
