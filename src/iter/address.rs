use crate::NetAddr;
use crate::Netv4Addr;
use crate::Netv6Addr;

use crate::Contains;

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

pub struct NetAddrAddressIterator {
	net: NetAddr,
	cur: Option<IpAddr>,
}

impl Iterator for NetAddrAddressIterator {
	type Item = IpAddr;

	fn next(&mut self) -> Option<Self::Item> {
		let cur: Option<IpAddr> = self.cur;
		let next: Option<IpAddr> = match cur {
			Some(IpAddr::V4(ipv4)) => u32::from(ipv4)
				.checked_add(1)
				.map(|next: u32| IpAddr::V4(Ipv4Addr::from(next))),
			Some(IpAddr::V6(ipv6)) => u128::from(ipv6)
				.checked_add(1)
				.map(|next: u128| IpAddr::V6(Ipv6Addr::from(next))),
			None => None,
		};

		match (cur, next) {
			(Some(cur), Some(next)) => {
				if self.net.contains(&cur) {
					self.cur = Some(next);
					Some(cur)
				} else {
					None
				}
			}
			(Some(cur), None) => {
				self.cur = next;
				Some(cur)
			}
			(None, _) => None,
		}
	}
}

pub struct Netv4AddrAddressIterator {
	net: Netv4Addr,
	cur: Ipv4Addr,
}

pub struct Netv6AddrAddressIterator {
	net: Netv6Addr,
	cur: Ipv6Addr,
}

#[cfg(test)]
mod tests {
	use super::NetAddrAddressIterator;
	use crate::NetAddr;
	use std::net::IpAddr;

	impl crate::NetAddr {
		pub fn iter(&self) -> NetAddrAddressIterator {
			NetAddrAddressIterator {
				net: *self,
				cur: Some(self.addr())
			}
		}
	}

	mod v4 {
		use super::*;

		#[test]
		fn loopback_slash_32_produces_one_off() {
			let net: NetAddr = "127.0.16.0/32".parse().unwrap();

			let mut iterator: NetAddrAddressIterator = net.iter();

			assert_eq!(
				iterator.next(),
				Some("127.0.16.0".parse::<IpAddr>().unwrap())
			);

			assert_eq!(iterator.next(), None);
		}

		#[test]
		fn loopback_slash_29_produces_all_ips_in_network() {
			let net: NetAddr = "127.0.16.0/29".parse().unwrap();

			let mut iterator: NetAddrAddressIterator = net.iter();

			assert_eq!(
				iterator.next(),
				Some("127.0.16.0".parse::<IpAddr>().unwrap())
			);
			assert_eq!(
				iterator.next(),
				Some("127.0.16.1".parse::<IpAddr>().unwrap())
			);
			assert_eq!(
				iterator.next(),
				Some("127.0.16.2".parse::<IpAddr>().unwrap())
			);
			assert_eq!(
				iterator.next(),
				Some("127.0.16.3".parse::<IpAddr>().unwrap())
			);
			assert_eq!(
				iterator.next(),
				Some("127.0.16.4".parse::<IpAddr>().unwrap())
			);
			assert_eq!(
				iterator.next(),
				Some("127.0.16.5".parse::<IpAddr>().unwrap())
			);
			assert_eq!(
				iterator.next(),
				Some("127.0.16.6".parse::<IpAddr>().unwrap())
			);
			assert_eq!(
				iterator.next(),
				Some("127.0.16.7".parse::<IpAddr>().unwrap())
			);

			assert_eq!(iterator.next(), None);
		}

		#[test]
		fn loopback_max_value_properly_stops() {
			let net: NetAddr = "255.255.255.255/31".parse().unwrap();

			let mut iterator: NetAddrAddressIterator = net.iter();

			assert_eq!(
				iterator.next(),
				Some("255.255.255.254".parse::<IpAddr>().unwrap())
			);
			assert_eq!(
				iterator.next(), Some("255.255.255.255".parse::<IpAddr>().unwrap())
			);

			assert_eq!(iterator.next(), None);
		}
	}
}
