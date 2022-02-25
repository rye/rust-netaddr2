use crate::Contains;

use crate::{Netv4Addr, Netv6Addr};

use super::offset::Offset;

pub struct SubnetIterator<Network, Subnet> {
	net: Network,
	cur: Option<Subnet>,
}

impl Iterator for SubnetIterator<Netv4Addr, Netv4Addr> {
	type Item = Netv4Addr;

	fn next(&mut self) -> Option<Self::Item> {
		match self.cur {
			// If self.cur exists and is contained in self.net, it _might_ be a
			// subnet.
			Some(cur) if self.net.contains(&cur) => {
				let device_count: Option<u32> = cur.len();

				let next: Option<Netv4Addr> = device_count
					.map(|device_count: u32| cur.offset(device_count))
					.flatten();

				match next {
					Some(next) => {
						if self.net.contains(&next) {
							self.cur = Some(next);
							Some(cur)
						} else {
							self.cur = None;
							Some(cur)
						}
					}
					None => {
						self.cur = None;
						Some(cur)
					}
				}
			}
			// If self.cur exists but isn't contained in self.net, it isn't a subnet.
			Some(_) => None,
			None => None,
		}
	}
}

impl Iterator for SubnetIterator<Netv6Addr, Netv6Addr> {
	type Item = Netv6Addr;

	fn next(&mut self) -> Option<Self::Item> {
		match self.cur {
			// If self.cur exists and is contained in self.net, it _might_ be a
			// subnet.
			Some(cur) if self.net.contains(&cur) => {
				let device_count: Option<u128> = cur.len();

				let next: Option<Netv6Addr> = device_count
					.map(|device_count: u128| cur.offset(device_count))
					.flatten();

				match next {
					Some(next) => {
						if self.net.contains(&next) {
							self.cur = Some(next);
							Some(cur)
						} else {
							self.cur = None;
							Some(cur)
						}
					}
					None => {
						self.cur = None;
						Some(cur)
					}
				}
			}
			// If self.cur exists but isn't contained in self.net, it isn't a subnet.
			Some(_) => None,
			None => None,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::SubnetIterator;

	mod netv4addr {
		use super::*;

		use crate::Netv4Addr;
		use std::net::Ipv4Addr;

		impl crate::Netv4Addr {
			pub fn subnets(&self, mask: Ipv4Addr) -> SubnetIterator<Netv4Addr, Netv4Addr> {
				SubnetIterator {
					net: *self,
					cur: Some(Netv4Addr::new(self.addr(), mask)),
				}
			}
		}

		#[test]
		fn larger_network_returns_none() {
			let outer: Netv4Addr = "127.0.16.0/24".parse().unwrap();
			// actually significantly larger than `outer`'s mask
			let mask: Ipv4Addr = "255.0.0.0".parse().unwrap();

			let mut it: SubnetIterator<Netv4Addr, Netv4Addr> = outer.subnets(mask);
			assert_eq!(it.next(), None);
		}

		#[test]
		fn same_sized_returns_once() {
			let outer: Netv4Addr = "127.16.32.0/24".parse().unwrap();

			let mask: Ipv4Addr = "255.255.255.0".parse().unwrap();

			let mut it: SubnetIterator<Netv4Addr, Netv4Addr> = outer.subnets(mask);
			assert_eq!(it.next(), Some("127.16.32.0/24".parse().unwrap()));
			assert_eq!(it.next(), None);
		}

		#[test]
		fn half_sized_returns_twice() {
			let outer: Netv4Addr = "127.16.32.0/24".parse().unwrap();

			let mask: Ipv4Addr = "255.255.255.128".parse().unwrap();

			let mut it: SubnetIterator<Netv4Addr, Netv4Addr> = outer.subnets(mask);
			assert_eq!(it.next(), Some("127.16.32.0/25".parse().unwrap()));
			assert_eq!(it.next(), Some("127.16.32.128/25".parse().unwrap()));
			assert_eq!(it.next(), None);
		}

		#[test]
		fn quarter_sized_returns_four_times() {
			let outer: Netv4Addr = "127.16.32.0/24".parse().unwrap();

			let mask: Ipv4Addr = "255.255.255.192".parse().unwrap();

			let mut it: SubnetIterator<Netv4Addr, Netv4Addr> = outer.subnets(mask);
			assert_eq!(it.next(), Some("127.16.32.0/26".parse().unwrap()));
			assert_eq!(it.next(), Some("127.16.32.64/26".parse().unwrap()));
			assert_eq!(it.next(), Some("127.16.32.128/26".parse().unwrap()));
			assert_eq!(it.next(), Some("127.16.32.192/26".parse().unwrap()));
			assert_eq!(it.next(), None);
		}

		#[test]
		fn eighth_sized_returns_eight_times() {
			let outer: Netv4Addr = "127.16.32.0/24".parse().unwrap();

			let mask: Ipv4Addr = "255.255.255.224".parse().unwrap();

			let mut it: SubnetIterator<Netv4Addr, Netv4Addr> = outer.subnets(mask);
			assert_eq!(it.next(), Some("127.16.32.0/27".parse().unwrap()));
			assert_eq!(it.next(), Some("127.16.32.32/27".parse().unwrap()));
			assert_eq!(it.next(), Some("127.16.32.64/27".parse().unwrap()));
			assert_eq!(it.next(), Some("127.16.32.96/27".parse().unwrap()));
			assert_eq!(it.next(), Some("127.16.32.128/27".parse().unwrap()));
			assert_eq!(it.next(), Some("127.16.32.160/27".parse().unwrap()));
			assert_eq!(it.next(), Some("127.16.32.192/27".parse().unwrap()));
			assert_eq!(it.next(), Some("127.16.32.224/27".parse().unwrap()));
			assert_eq!(it.next(), None);
		}
	}

	mod netv6addr {
		use super::*;

		use crate::Netv6Addr;
		use std::net::Ipv6Addr;

		impl crate::Netv6Addr {
			pub fn subnets(&self, mask: Ipv6Addr) -> SubnetIterator<Netv6Addr, Netv6Addr> {
				SubnetIterator {
					net: *self,
					cur: Some(Netv6Addr::new(self.addr(), mask)),
				}
			}
		}

		#[test]
		fn larger_network_returns_none() {
			let outer: Netv6Addr = "2001:db8:dead:beef::/64".parse().unwrap();
			// actually significantly larger than `outer`'s mask
			let mask: Ipv6Addr = "ffff:ffff::".parse().unwrap();

			let mut it: SubnetIterator<Netv6Addr, Netv6Addr> = outer.subnets(mask);
			assert_eq!(it.next(), None);
		}

		#[test]
		fn same_sized_returns_once() {
			let outer: Netv6Addr = "2001:db8:dead:beef::/64".parse().unwrap();

			let mask: Ipv6Addr = "ffff:ffff:ffff:ffff::".parse().unwrap();

			let mut it: SubnetIterator<Netv6Addr, Netv6Addr> = outer.subnets(mask);
			assert_eq!(it.next(), Some("2001:db8:dead:beef::/64".parse().unwrap()));
			assert_eq!(it.next(), None);
		}

		#[test]
		fn half_sized_returns_twice() {
			let outer: Netv6Addr = "2001:db8:dead:beef::/63".parse().unwrap();

			let mask: Ipv6Addr = "ffff:ffff:ffff:ffff::".parse().unwrap();

			let mut it: SubnetIterator<Netv6Addr, Netv6Addr> = outer.subnets(mask);
			assert_eq!(it.next(), "2001:db8:dead:beee::/64".parse().ok());
			assert_eq!(it.next(), "2001:db8:dead:beef::/64".parse().ok());
			assert_eq!(it.next(), None);
		}

		#[test]
		fn quarter_sized_returns_four_times() {
			let outer: Netv6Addr = "2001:db8:dead:beef::/62".parse().unwrap();

			let mask: Ipv6Addr = "ffff:ffff:ffff:ffff::".parse().unwrap();

			let mut it: SubnetIterator<Netv6Addr, Netv6Addr> = outer.subnets(mask);
			assert_eq!(it.next(), "2001:db8:dead:beec::/64".parse().ok());
			assert_eq!(it.next(), "2001:db8:dead:beed::/64".parse().ok());
			assert_eq!(it.next(), "2001:db8:dead:beee::/64".parse().ok());
			assert_eq!(it.next(), "2001:db8:dead:beef::/64".parse().ok());
			assert_eq!(it.next(), None);
		}

		#[test]
		fn eighth_sized_returns_eight_times() {
			let outer: Netv6Addr = "2001:db8:dead:beef::/61".parse().unwrap();

			let mask: Ipv6Addr = "ffff:ffff:ffff:ffff::".parse().unwrap();

			let mut it: SubnetIterator<Netv6Addr, Netv6Addr> = outer.subnets(mask);
			assert_eq!(it.next(), "2001:db8:dead:bee8::/64".parse().ok());
			assert_eq!(it.next(), "2001:db8:dead:bee9::/64".parse().ok());
			assert_eq!(it.next(), "2001:db8:dead:beea::/64".parse().ok());
			assert_eq!(it.next(), "2001:db8:dead:beeb::/64".parse().ok());
			assert_eq!(it.next(), "2001:db8:dead:beec::/64".parse().ok());
			assert_eq!(it.next(), "2001:db8:dead:beed::/64".parse().ok());
			assert_eq!(it.next(), "2001:db8:dead:beee::/64".parse().ok());
			assert_eq!(it.next(), "2001:db8:dead:beef::/64".parse().ok());
			assert_eq!(it.next(), None);
		}
	}
}
