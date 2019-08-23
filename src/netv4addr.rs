use crate::traits::Mask;
use std::net::Ipv4Addr;

/// A structure representing an IPv4 network.
///
/// Internally, this structure includes two values; an Ipv4Addr
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
