use core::cmp::Ordering;

use crate::netv4addr::Netv4Addr;

impl Ord for Netv4Addr {
	fn cmp(&self, other: &Self) -> Ordering {
		match self.addr().cmp(&other.addr()) {
			Ordering::Equal => self.mask().cmp(&other.mask()),
			ordering => ordering,
		}
	}
}

#[cfg(test)]
mod tests {
	use core::cmp::Ordering;

	use crate::netv4addr::Netv4Addr;

	#[test]
	fn different_networks() {
		let a: Netv4Addr = "1.0.0.0/8".parse().unwrap();
		let b: Netv4Addr = "2.0.0.0/8".parse().unwrap();

		assert_eq!(a.cmp(&b), Ordering::Less);
	}

	#[test]
	fn different_netmasks() {
		let a: Netv4Addr = "1.0.0.0/7".parse().unwrap();
		let b: Netv4Addr = "1.0.0.0/8".parse().unwrap();

		assert_eq!(a.cmp(&b), Ordering::Less);
	}

	#[test]
	fn different() {
		let a: Netv4Addr = "1.0.0.0/8".parse().unwrap();
		let b: Netv4Addr = "0.0.0.0/24".parse().unwrap();

		assert_eq!(a.cmp(&b), Ordering::Greater);
	}

	#[test]
	fn equal() {
		let a: Netv4Addr = "1.0.0.0/8".parse().unwrap();
		let b: Netv4Addr = "1.0.0.0/8".parse().unwrap();

		assert_eq!(a.cmp(&b), Ordering::Equal);
	}
}
