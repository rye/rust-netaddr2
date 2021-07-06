use super::Netv4Addr;
use core::cmp::Ordering;

impl PartialOrd for Netv4Addr {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		match self.addr().partial_cmp(&other.addr()) {
			Some(Ordering::Equal) => self.mask().partial_cmp(&other.mask()),
			Some(ordering) => Some(ordering),
			None => None,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::Netv4Addr;
	use core::cmp::Ordering;

	#[test]
	fn different_networks() {
		let a: Netv4Addr = "1.0.0.0/8".parse().unwrap();
		let b: Netv4Addr = "2.0.0.0/8".parse().unwrap();

		assert_eq!(a.partial_cmp(&b), Some(Ordering::Less))
	}

	#[test]
	fn different_netmasks() {
		let a: Netv4Addr = "1.0.0.0/7".parse().unwrap();
		let b: Netv4Addr = "1.0.0.0/8".parse().unwrap();

		assert_eq!(a.partial_cmp(&b), Some(Ordering::Less))
	}

	#[test]
	fn different() {
		let a: Netv4Addr = "1.0.0.0/8".parse().unwrap();
		let b: Netv4Addr = "0.0.0.0/24".parse().unwrap();

		assert_eq!(a.partial_cmp(&b), Some(Ordering::Greater))
	}

	#[test]
	fn equal() {
		let a: Netv4Addr = "1.0.0.0/8".parse().unwrap();
		let b: Netv4Addr = "1.0.0.0/8".parse().unwrap();

		assert_eq!(a.partial_cmp(&b), Some(Ordering::Equal))
	}
}
