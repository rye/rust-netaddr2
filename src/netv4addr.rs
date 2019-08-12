use crate::merge::Merge;
use crate::mask::Mask;
use core::cmp::Ordering;
use std::net::Ipv4Addr;

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

impl Merge for Netv4Addr {
	type Output = Option<Self>;

	fn merge(&self, other: &Self) -> Self::Output {
		let addr: u32 = self.addr().clone().into();
		let mask: u32 = self.mask().clone().into();
		let other_addr: u32 = other.addr().clone().into();
		let other_mask: u32 = other.mask().clone().into();

		let mask: u32 = match mask.cmp(&other_mask) {
			Ordering::Equal => mask << 1,
			Ordering::Less => mask,
			Ordering::Greater => other_mask,
		};

		if addr & mask == other_addr & mask {
			Some(Self::new(Ipv4Addr::from(addr & mask), Ipv4Addr::from(mask)))
		} else {
			None
		}
	}
}
