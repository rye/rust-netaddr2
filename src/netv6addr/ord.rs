use crate::netv6addr::Netv6Addr;
use core::cmp::Ordering;

impl Ord for Netv6Addr {
	fn cmp(&self, other: &Self) -> Ordering {
		match self.addr().cmp(other.addr()) {
			Ordering::Equal => self.mask().cmp(other.mask()),
			ordering => ordering,
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

		assert_eq!(a.cmp(&b), Ordering::Less)
	}

	#[test]
	fn different_netmasks() {
		let a: Netv6Addr = "2001:db8:0:0::0/63".parse().unwrap();
		let b: Netv6Addr = "2001:db8:0:0::0/64".parse().unwrap();

		assert_eq!(a.cmp(&b), Ordering::Less)
	}

	#[test]
	fn different() {
		let a: Netv6Addr = "ff02::1/16".parse().unwrap();
		let b: Netv6Addr = "2001:db8:0:1::0/64".parse().unwrap();

		assert_eq!(a.cmp(&b), Ordering::Greater)
	}

	#[test]
	fn equal() {
		let a: Netv6Addr = "2001:db8:dead:beef::0/64".parse().unwrap();
		let b: Netv6Addr = "2001:db8:dead:beef::0/64".parse().unwrap();

		assert_eq!(a.cmp(&b), Ordering::Equal)
	}
}
