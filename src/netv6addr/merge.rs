use crate::netv6addr::Netv6Addr;
use crate::traits::Merge;
use core::cmp::Ordering;
use std::net::Ipv6Addr;

impl Merge for Netv6Addr {
	type Output = Option<Self>;

	fn merge(&self, other: &Self) -> Self::Output {
		let addr: u128 = self.addr().clone().into();
		let mask: u128 = self.mask().clone().into();
		let other_addr: u128 = other.addr().clone().into();
		let other_mask: u128 = other.mask().clone().into();

		let mask: u128 = match mask.cmp(&other_mask) {
			Ordering::Equal => mask << 1,
			Ordering::Less => mask,
			Ordering::Greater => other_mask,
		};

		if addr & mask == other_addr & mask {
			Some(Self::new(Ipv6Addr::from(addr & mask), Ipv6Addr::from(mask)))
		} else {
			None
		}
	}
}
