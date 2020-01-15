use crate::Contains;

use crate::Netv4Addr;

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
					cur: Some(Netv4Addr::new(*self.addr(), mask)),
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
}
