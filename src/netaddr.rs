use crate::Mask;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

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
	pub const F32: u32 = u32::max_value();
	pub const F128: u128 = u128::max_value();
	pub const F32V4: Ipv4Addr = Ipv4Addr::new(255, 255, 255, 255);
	pub const F32V6: Ipv6Addr = Ipv6Addr::new(
		0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff,
	);
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Ord)]
pub struct Netv4Addr {
	mask: Ipv4Addr,
	addr: Ipv4Addr,
}

impl Netv4Addr {
	pub(crate) fn mask(&self) -> &Ipv4Addr {
		&self.mask
	}

	pub(crate) fn addr(&self) -> &Ipv4Addr {
		&self.addr
	}

	pub fn new(addr: Ipv4Addr, mask: Ipv4Addr) -> Self {
		let addr = addr.mask(&mask);
		Self { addr, mask }
	}
}

impl From<(Ipv4Addr, Ipv4Addr)> for Netv4Addr {
	fn from((addr, mask): (Ipv4Addr, Ipv4Addr)) -> Self {
		Self::new(addr, mask)
	}
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Ord)]
pub struct Netv6Addr {
	mask: Ipv6Addr,
	addr: Ipv6Addr,
}

impl Netv6Addr {
	pub(crate) fn mask(&self) -> &Ipv6Addr {
		&self.mask
	}

	pub(crate) fn addr(&self) -> &Ipv6Addr {
		&self.addr
	}

	pub fn new(addr: Ipv6Addr, mask: Ipv6Addr) -> Self {
		Self::from((addr, mask))
	}
}

impl From<(Ipv6Addr, Ipv6Addr)> for Netv6Addr {
	fn from(addr_mask: (Ipv6Addr, Ipv6Addr)) -> Self {
		let (addr, mask): (Ipv6Addr, Ipv6Addr) = addr_mask;
		let addr = addr.mask(&mask);
		Self { addr, mask }
	}
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
