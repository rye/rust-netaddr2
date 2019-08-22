#[cfg(test)]
mod tests {
	use crate::NetAddr;
	use std::cmp::Ordering;

	mod v4 {
		use super::*;

		#[test]
		fn different_networks() {
			let a: NetAddr = "1.0.0.0/8".parse().unwrap();
			let b: NetAddr = "2.0.0.0/8".parse().unwrap();

			assert_eq!(a.partial_cmp(&b), Some(Ordering::Less))
		}

		#[test]
		fn different_netmasks() {
			let a: NetAddr = "1.0.0.0/7".parse().unwrap();
			let b: NetAddr = "1.0.0.0/8".parse().unwrap();

			assert_eq!(a.partial_cmp(&b), Some(Ordering::Less))
		}

		#[test]
		fn different() {
			let a: NetAddr = "1.0.0.0/8".parse().unwrap();
			let b: NetAddr = "0.0.0.0/24".parse().unwrap();

			assert_eq!(a.partial_cmp(&b), Some(Ordering::Greater))
		}

		#[test]
		fn equal() {
			let a: NetAddr = "1.0.0.0/8".parse().unwrap();
			let b: NetAddr = "1.0.0.0/8".parse().unwrap();

			assert_eq!(a.partial_cmp(&b), Some(Ordering::Equal))
		}
	}

	mod v6 {
		use super::*;

		#[test]
		fn different_networks() {
			let a: NetAddr = "2001:db8:0:0::0/64".parse().unwrap();
			let b: NetAddr = "2001:db8:0:1::0/64".parse().unwrap();

			assert_eq!(a.partial_cmp(&b), Some(Ordering::Less))
		}

		#[test]
		fn different_netmasks() {
			let a: NetAddr = "2001:db8:0:0::0/63".parse().unwrap();
			let b: NetAddr = "2001:db8:0:0::0/64".parse().unwrap();

			assert_eq!(a.partial_cmp(&b), Some(Ordering::Less))
		}

		#[test]
		fn different() {
			let a: NetAddr = "ff02::1/16".parse().unwrap();
			let b: NetAddr = "2001:db8:0:1::0/64".parse().unwrap();

			assert_eq!(a.partial_cmp(&b), Some(Ordering::Greater))
		}

		#[test]
		fn equal() {
			let a: NetAddr = "2001:db8:dead:beef::0/64".parse().unwrap();
			let b: NetAddr = "2001:db8:dead:beef::0/64".parse().unwrap();

			assert_eq!(a.partial_cmp(&b), Some(Ordering::Equal))
		}
	}
}
