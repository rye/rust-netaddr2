use crate::NetAddr;
use crate::Netv4Addr;
use crate::Netv6Addr;

use crate::Contains;

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

pub struct AddressIterator<Network, Address> {
	net: Network,
	cur: Option<Address>,
}

impl Iterator for AddressIterator<NetAddr, IpAddr> {
	type Item = IpAddr;

	fn next(&mut self) -> Option<Self::Item> {
		let cur: Option<Self::Item> = self.cur;
		let next: Option<Self::Item> = match cur {
			Some(IpAddr::V4(v4)) => u32::from(v4)
				.checked_add(1)
				.map(|next: u32| IpAddr::V4(Ipv4Addr::from(next))),
			Some(IpAddr::V6(v6)) => u128::from(v6)
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

#[cfg(test)]
mod tests {
	use super::AddressIterator;
	use crate::NetAddr;
	use std::net::IpAddr;

	impl crate::NetAddr {
		pub fn iter(&self) -> AddressIterator<NetAddr, IpAddr> {
			AddressIterator {
				net: *self,
				cur: Some(self.addr()),
			}
		}
	}

	mod v4 {
		use super::*;

		#[test]
		fn loopback_slash_32_produces_one_off() {
			let net: NetAddr = "127.0.16.0/32".parse().unwrap();

			let mut iterator: AddressIterator<NetAddr, IpAddr> = net.iter();

			assert_eq!(
				iterator.next(),
				Some("127.0.16.0".parse::<IpAddr>().unwrap())
			);

			assert_eq!(iterator.next(), None);
		}

		#[test]
		fn loopback_slash_29_produces_all_ips_in_network() {
			let net: NetAddr = "127.0.16.0/29".parse().unwrap();

			let mut iterator: AddressIterator<NetAddr, IpAddr> = net.iter();

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

			let mut iterator: AddressIterator<NetAddr, IpAddr> = net.iter();

			assert_eq!(
				iterator.next(),
				Some("255.255.255.254".parse::<IpAddr>().unwrap())
			);
			assert_eq!(
				iterator.next(),
				Some("255.255.255.255".parse::<IpAddr>().unwrap())
			);

			assert_eq!(iterator.next(), None);
		}
	}
}
