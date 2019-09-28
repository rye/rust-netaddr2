use super::Netv4Addr;
use crate::traits::Merge;
use core::cmp::Ordering;
use std::net::Ipv4Addr;

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

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn mergeable_networks_correct() {
		let a: Netv4Addr = "10.0.0.0/24".parse().unwrap();
		let b: Netv4Addr = "10.0.1.0/24".parse().unwrap();

		assert_eq!(a.merge(&b), Some("10.0.0.0/23".parse().unwrap()));
	}

	#[test]
	fn mergeable_networks_reflexive() {
		let a: Netv4Addr = "10.0.0.0/24".parse().unwrap();
		let b: Netv4Addr = "10.0.1.0/24".parse().unwrap();

		assert_eq!(a.merge(&b), b.merge(&a));
	}

	#[test]
	fn nested_networks_takes_biggest() {
		let a: Netv4Addr = "10.0.0.0/24".parse().unwrap();
		let b: Netv4Addr = "10.0.0.0/23".parse().unwrap();

		assert_eq!(a.merge(&b), Some(b));
	}

	#[test]
	fn nested_networks_reflexive() {
		let a: Netv4Addr = "10.0.0.0/24".parse().unwrap();
		let b: Netv4Addr = "10.0.0.0/23".parse().unwrap();

		assert_eq!(a.merge(&b), b.merge(&a));
	}

	#[test]
	fn adjacent_but_not_mergable_none() {
		let a: Netv4Addr = "10.0.1.0/24".parse().unwrap();
		let b: Netv4Addr = "10.0.2.0/24".parse().unwrap();

		assert_eq!(a.merge(&b), None);
		assert_eq!(b.merge(&a), None);
		assert_eq!(a.merge(&b), b.merge(&a));
	}
}
