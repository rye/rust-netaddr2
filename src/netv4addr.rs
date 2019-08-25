use crate::traits::Mask;
use std::net::Ipv4Addr;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Ord)]
pub struct Netv4Addr {
	mask: Ipv4Addr,
	addr: Ipv4Addr,
}

impl Netv4Addr {
	pub(crate) const fn mask(&self) -> &Ipv4Addr {
		&self.mask
	}

	pub(crate) const fn addr(&self) -> &Ipv4Addr {
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
