use crate::traits::Mask;
use std::net::Ipv4Addr;

/// A structure representing an IPv4 network.
///
/// Internally, this structure includes two values; an `Ipv4Addr`
/// representing the network address (`addr`), and another
/// representing the netmask (`mask`).
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Ord)]
pub struct Netv4Addr {
	mask: Ipv4Addr,
	addr: Ipv4Addr,
}

impl Netv4Addr {
	pub const fn mask(&self) -> &Ipv4Addr {
		&self.mask
	}

	pub const fn addr(&self) -> &Ipv4Addr {
		&self.addr
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
	/// ```
	pub fn new(addr: Ipv4Addr, mask: Ipv4Addr) -> Self {
		let addr = addr.mask(&mask);
		Self { addr, mask }
	}
}

mod broadcast;
mod contains;
mod from;
mod fromstr;
mod hash;
mod merge;
mod partialord;
