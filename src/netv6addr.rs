use crate::traits::Mask;
use std::net::Ipv6Addr;

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

	pub fn new(addr: Ipv6Addr, mask: Ipv6Addr) -> Self {
		let addr = addr.mask(&mask);
		Self { addr, mask }
	}
}

mod contains;
mod from;
mod fromstr;
mod hash;
mod merge;
mod partialord;
