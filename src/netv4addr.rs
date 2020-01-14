use crate::traits::Mask;
use std::net::Ipv4Addr;

/// A structure representing an IPv4 network.
///
/// Internally, this structure includes two values; an `Ipv4Addr` representing
/// the network address (`addr`), and another representing the netmask (`mask`).
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Ord)]
pub struct Netv4Addr {
	addr: Ipv4Addr,
	mask: Ipv4Addr,
}

impl Netv4Addr {
	pub const fn mask(&self) -> Ipv4Addr {
		self.mask
	}

	pub const fn addr(&self) -> Ipv4Addr {
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
	pub fn new(addr: Ipv4Addr, mask: Ipv4Addr) -> Self {
		let addr = addr.mask(&mask);
		Self { addr, mask }
	}

	/// Compute the number of addresses in this network.
	///
	/// This is done by raising `2_u32` to the number of zeroes in the netmask,
	/// which fails if the number of zeros in the netmask is the _length_ of the
	/// netmask.  Computing this result in this way maintains compatibility with
	/// non-CIDR netmasks.
	///
	/// # Examples
	///
	/// First, a "typical" example.
	///
	/// ```rust
	/// # use netaddr2::Netv4Addr;
	/// let netaddr: Netv4Addr = "127.16.32.0/16".parse().unwrap();
	/// assert_eq!(netaddr.len(), Some(65536_u32));
	/// ```
	///
	/// Note that a `/32` has exactly 2^(0 zeros) = 1 addresses in it.
	///
	/// ```rust
	/// # use netaddr2::Netv4Addr;
	/// let netaddr: Netv4Addr = "127.16.32.0/32".parse().unwrap();
	/// assert_eq!(netaddr.len(), Some(1_u32));
	/// ```
	///
	/// And there are not enough bits to store the number of devices in a `/0`, so
	/// the length is not definable.  (This should be the only case when `None`
	/// gets returned.)
	///
	/// ```rust
	/// # use netaddr2::Netv4Addr;
	/// let netaddr: Netv4Addr = "0.0.0.0/0".parse().unwrap();
	/// assert_eq!(netaddr.len(), None);
	/// ```
	pub fn len(&self) -> Option<u32> {
		2_u32.checked_pow(u32::from(self.mask).count_zeros())
	}

	/// Determine if the network is empty.
	///
	/// (Plot twist, it isn't.)  Even a /32 has one device in it.
	pub const fn is_empty(&self) -> bool {
		false
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
	use super::*;

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

	mod len {
		use super::*;

		#[test]
		fn len_correct_max_mask() {
			let netaddr: Netv4Addr = "0.0.0.0/32".parse().unwrap();
			assert_eq!(netaddr.len(), Some(1));
		}

		#[test]
		fn len_correct_min_mask() {
			let netaddr: Netv4Addr = "0.0.0.0/0".parse().unwrap();
			assert_eq!(netaddr.len(), None);
		}
	}

	mod is_empty {
		use super::*;

		#[test]
		fn is_false() {
			let netaddr: Netv4Addr = "0.0.0.0/0".parse().unwrap();
			assert!(!netaddr.is_empty());
		}
	}
}
