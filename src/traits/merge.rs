pub trait Merge {
	type Output;

	fn merge(&self, other: &Self) -> Self::Output;
}

#[cfg(test)]
mod tests {
	use super::Merge;

	mod netaddr {
		use super::Merge;
		use crate::NetAddr;

		mod v4 {
			use super::*;

			#[test]
			fn mergeable_networks_correct() {
				let a: NetAddr = "10.0.0.0/24".parse().unwrap();
				let b: NetAddr = "10.0.1.0/24".parse().unwrap();

				assert_eq!(a.merge(&b), Some("10.0.0.0/23".parse().unwrap()));
			}

			#[test]
			fn mergeable_networks_reflexive() {
				let a: NetAddr = "10.0.0.0/24".parse().unwrap();
				let b: NetAddr = "10.0.1.0/24".parse().unwrap();

				assert_eq!(a.merge(&b), b.merge(&a));
			}

			#[test]
			fn nested_networks_takes_biggest() {
				let a: NetAddr = "10.0.0.0/24".parse().unwrap();
				let b: NetAddr = "10.0.0.0/23".parse().unwrap();

				assert_eq!(a.merge(&b), Some(b));
			}

			#[test]
			fn nested_networks_reflexive() {
				let a: NetAddr = "10.0.0.0/24".parse().unwrap();
				let b: NetAddr = "10.0.0.0/23".parse().unwrap();

				assert_eq!(a.merge(&b), b.merge(&a));
			}

			#[test]
			fn adjacent_but_not_mergable_none() {
				let a: NetAddr = "10.0.1.0/24".parse().unwrap();
				let b: NetAddr = "10.0.2.0/24".parse().unwrap();

				assert_eq!(a.merge(&b), None);
				assert_eq!(b.merge(&a), None);
				assert_eq!(a.merge(&b), b.merge(&a));
			}
		}

		mod v6 {
			use super::*;

			#[test]
			fn mergeable_networks_correct() {
				let a: NetAddr = "2001:db8:dead:beef::/64".parse().unwrap();
				let b: NetAddr = "2001:db8:dead:beee::/64".parse().unwrap();

				assert_eq!(
					a.merge(&b),
					Some("2001:db8:dead:beee::/63".parse().unwrap())
				);
			}

			#[test]
			fn mergeable_networks_reflexive() {
				let a: NetAddr = "2001:db8:dead:beef::/64".parse().unwrap();
				let b: NetAddr = "2001:db8:dead:beee::/64".parse().unwrap();

				assert_eq!(a.merge(&b), b.merge(&a));
			}

			#[test]
			fn nested_networks_takes_biggest() {
				let a: NetAddr = "2001:db8:dead:beee::/63".parse().unwrap();
				let b: NetAddr = "2001:db8:dead:beef::/64".parse().unwrap();

				assert_eq!(a.merge(&b), Some(a));
			}

			#[test]
			fn nested_networks_reflexive() {
				let a: NetAddr = "2001:db8:dead:beee::/63".parse().unwrap();
				let b: NetAddr = "2001:db8:dead:beef::/64".parse().unwrap();

				assert_eq!(a.merge(&b), b.merge(&a));
			}

			#[test]
			fn adjacent_but_not_mergable_none() {
				let a: NetAddr = "2001:db8:dead:beee::/64".parse().unwrap();
				let b: NetAddr = "2001:db8:dead:beed::/64".parse().unwrap();

				assert_eq!(a.merge(&b), None);
				assert_eq!(b.merge(&a), None);
				assert_eq!(a.merge(&b), b.merge(&a));
			}
		}
	}
}
