use crate::netv6addr::Netv6Addr;
use core::cmp::Ordering;

impl PartialOrd for Netv6Addr {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		match self.addr().partial_cmp(other.addr()) {
			Some(Ordering::Equal) => self.mask().partial_cmp(other.mask()),
			Some(ordering) => Some(ordering),
			None => None,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::Netv6Addr;
	use core::cmp::Ordering;

	#[test]
	fn different_networks() {
		let a: Netv6Addr = "2001:db8:0:0::0/64".parse().unwrap();
		let b: Netv6Addr = "2001:db8:0:1::0/64".parse().unwrap();

		assert_eq!(a.partial_cmp(&b), Some(Ordering::Less))
	}

	#[test]
	fn different_netmasks() {
		let a: Netv6Addr = "2001:db8:0:0::0/63".parse().unwrap();
		let b: Netv6Addr = "2001:db8:0:0::0/64".parse().unwrap();

		assert_eq!(a.partial_cmp(&b), Some(Ordering::Less))
	}

	#[test]
	fn different() {
		let a: Netv6Addr = "ff02::1/16".parse().unwrap();
		let b: Netv6Addr = "2001:db8:0:1::0/64".parse().unwrap();

		assert_eq!(a.partial_cmp(&b), Some(Ordering::Greater))
	}

	#[test]
	fn equal() {
		let a: Netv6Addr = "2001:db8:dead:beef::0/64".parse().unwrap();
		let b: Netv6Addr = "2001:db8:dead:beef::0/64".parse().unwrap();

		assert_eq!(a.partial_cmp(&b), Some(Ordering::Equal))
	}
}
