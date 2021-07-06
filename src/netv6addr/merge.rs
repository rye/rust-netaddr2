use super::Netv6Addr;
use crate::traits::Merge;
use core::cmp::Ordering;
use std::net::Ipv6Addr;

impl Merge for Netv6Addr {
	type Output = Option<Self>;

	fn merge(&self, other: &Self) -> Self::Output {
		let addr: u128 = (*self.addr()).into();
		let mask: u128 = (*self.mask()).into();
		let other_addr: u128 = (*other.addr()).into();
		let other_mask: u128 = (*other.mask()).into();

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

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn mergeable_networks_correct() {
		let a: Netv6Addr = "2001:db8:dead:beef::/64".parse().unwrap();
		let b: Netv6Addr = "2001:db8:dead:beee::/64".parse().unwrap();

		assert_eq!(
			a.merge(&b),
			Some("2001:db8:dead:beee::/63".parse().unwrap())
		);
	}

	#[test]
	fn mergeable_networks_reflexive() {
		let a: Netv6Addr = "2001:db8:dead:beef::/64".parse().unwrap();
		let b: Netv6Addr = "2001:db8:dead:beee::/64".parse().unwrap();

		assert_eq!(a.merge(&b), b.merge(&a));
	}

	#[test]
	fn nested_networks_takes_biggest() {
		let a: Netv6Addr = "2001:db8:dead:beee::/63".parse().unwrap();
		let b: Netv6Addr = "2001:db8:dead:beef::/64".parse().unwrap();

		assert_eq!(a.merge(&b), Some(a));
	}

	#[test]
	fn nested_networks_reflexive() {
		let a: Netv6Addr = "2001:db8:dead:beee::/63".parse().unwrap();
		let b: Netv6Addr = "2001:db8:dead:beef::/64".parse().unwrap();

		assert_eq!(a.merge(&b), b.merge(&a));
	}

	#[test]
	fn adjacent_but_not_mergable_none() {
		let a: Netv6Addr = "2001:db8:dead:beee::/64".parse().unwrap();
		let b: Netv6Addr = "2001:db8:dead:beed::/64".parse().unwrap();

		assert_eq!(a.merge(&b), None);
		assert_eq!(b.merge(&a), None);
		assert_eq!(a.merge(&b), b.merge(&a));
	}
}
