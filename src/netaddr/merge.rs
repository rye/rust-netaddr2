use crate::{netaddr::NetAddr, netv4addr::Netv4Addr, netv6addr::Netv6Addr, traits::Merge};

impl Merge for NetAddr {
	type Output = Option<Self>;

	fn merge(&self, other: &Self) -> Self::Output {
		match (self, other) {
			(Self::V4(a), Self::V4(b)) => a.merge(b).map(|netvxaddr: Netv4Addr| netvxaddr.into()),
			(Self::V6(a), Self::V6(b)) => a.merge(b).map(|netvxaddr: Netv6Addr| netvxaddr.into()),
			(_, _) => unimplemented!(),
		}
	}
}

#[cfg(test)]
mod tests {
	mod v4 {
		use crate::{netaddr::NetAddr, traits::Merge};

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
		use crate::{netaddr::NetAddr, traits::Merge};

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
