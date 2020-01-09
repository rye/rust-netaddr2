use crate::Contains;

pub struct AddressIterator<Network, Address> {
	net: Network,
	cur: Option<Address>,
}

impl<Network, Address> Iterator for AddressIterator<Network, Address>
where
	Address: Copy + super::offset::MaybeNext<u32>,
	Network: Copy + Contains,
	Network: From<Address>,
{
	type Item = Address;

	fn next(&mut self) -> Option<Self::Item> {
		let cur: Option<Self::Item> = self.cur;
		let next: Option<Self::Item> = cur.map(|cur| cur.maybe_next(1_u32)).flatten();

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

	mod netv4addr {
		use super::*;

		use crate::Netv4Addr;
		use std::net::Ipv4Addr;

		impl crate::Netv4Addr {
			pub fn iter(&self) -> AddressIterator<Netv4Addr, Ipv4Addr> {
				AddressIterator {
					net: *self,
					cur: Some(*self.addr()),
				}
			}
		}

		#[test]
		fn loopback_slash_32_produces_one_off() {
			let net: Netv4Addr = "127.0.16.0/32".parse().unwrap();

			let mut iterator: AddressIterator<Netv4Addr, Ipv4Addr> = net.iter();
			assert_eq!(iterator.next(), "127.0.16.0".parse::<Ipv4Addr>().ok());
			assert_eq!(iterator.next(), None);
		}

		#[test]
		fn loopback_slash_29_produces_one_off() {
			let net: Netv4Addr = "127.0.16.0/29".parse().unwrap();

			let mut iterator: AddressIterator<Netv4Addr, Ipv4Addr> = net.iter();
			assert_eq!(iterator.next(), "127.0.16.0".parse::<Ipv4Addr>().ok());
			assert_eq!(iterator.next(), "127.0.16.1".parse::<Ipv4Addr>().ok());
			assert_eq!(iterator.next(), "127.0.16.2".parse::<Ipv4Addr>().ok());
			assert_eq!(iterator.next(), "127.0.16.3".parse::<Ipv4Addr>().ok());
			assert_eq!(iterator.next(), "127.0.16.4".parse::<Ipv4Addr>().ok());
			assert_eq!(iterator.next(), "127.0.16.5".parse::<Ipv4Addr>().ok());
			assert_eq!(iterator.next(), "127.0.16.6".parse::<Ipv4Addr>().ok());
			assert_eq!(iterator.next(), "127.0.16.7".parse::<Ipv4Addr>().ok());
			assert_eq!(iterator.next(), None);
		}

		#[test]
		fn loopback_max_value_properly_stops() {
			let net: Netv4Addr = "255.255.255.255/31".parse().unwrap();

			let mut iterator: AddressIterator<Netv4Addr, Ipv4Addr> = net.iter();
			assert_eq!(iterator.next(), "255.255.255.254".parse::<Ipv4Addr>().ok());
			assert_eq!(iterator.next(), "255.255.255.255".parse::<Ipv4Addr>().ok());
			assert_eq!(iterator.next(), None);
		}
	}

	mod netaddr {
		use super::*;

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
				assert_eq!(iterator.next(), "127.0.16.0".parse::<IpAddr>().ok());
				assert_eq!(iterator.next(), None);
			}

			#[test]
			fn loopback_slash_29_produces_all_ips_in_network() {
				let net: NetAddr = "127.0.16.0/29".parse().unwrap();

				let mut iterator: AddressIterator<NetAddr, IpAddr> = net.iter();
				assert_eq!(iterator.next(), "127.0.16.0".parse::<IpAddr>().ok());
				assert_eq!(iterator.next(), "127.0.16.1".parse::<IpAddr>().ok());
				assert_eq!(iterator.next(), "127.0.16.2".parse::<IpAddr>().ok());
				assert_eq!(iterator.next(), "127.0.16.3".parse::<IpAddr>().ok());
				assert_eq!(iterator.next(), "127.0.16.4".parse::<IpAddr>().ok());
				assert_eq!(iterator.next(), "127.0.16.5".parse::<IpAddr>().ok());
				assert_eq!(iterator.next(), "127.0.16.6".parse::<IpAddr>().ok());
				assert_eq!(iterator.next(), "127.0.16.7".parse::<IpAddr>().ok());
				assert_eq!(iterator.next(), None);
			}

			#[test]
			fn loopback_max_value_properly_stops() {
				let net: NetAddr = "255.255.255.255/31".parse().unwrap();

				let mut iterator: AddressIterator<NetAddr, IpAddr> = net.iter();
				assert_eq!(iterator.next(), "255.255.255.254".parse::<IpAddr>().ok());
				assert_eq!(iterator.next(), "255.255.255.255".parse::<IpAddr>().ok());
				assert_eq!(iterator.next(), None);
			}
		}

		mod v6 {
			use super::*;

			#[test]
			#[test]
			fn slash_128_produces_one_off() {
				let net: NetAddr = "2001:db8::1/128".parse().unwrap();

				let mut iterator: AddressIterator<NetAddr, IpAddr> = net.iter();
				assert_eq!(iterator.next(), "2001:db8::1".parse::<IpAddr>().ok());
				assert_eq!(iterator.next(), None);
			}

			#[test]
			fn slash_125_produces_all_ips_in_network() {
				let net: NetAddr = "2001:db8::1/125".parse().unwrap();

				let mut iterator: AddressIterator<NetAddr, IpAddr> = net.iter();
				assert_eq!(iterator.next(), "2001:db8::0".parse::<IpAddr>().ok());
				assert_eq!(iterator.next(), "2001:db8::1".parse::<IpAddr>().ok());
				assert_eq!(iterator.next(), "2001:db8::2".parse::<IpAddr>().ok());
				assert_eq!(iterator.next(), "2001:db8::3".parse::<IpAddr>().ok());
				assert_eq!(iterator.next(), "2001:db8::4".parse::<IpAddr>().ok());
				assert_eq!(iterator.next(), "2001:db8::5".parse::<IpAddr>().ok());
				assert_eq!(iterator.next(), "2001:db8::6".parse::<IpAddr>().ok());
				assert_eq!(iterator.next(), "2001:db8::7".parse::<IpAddr>().ok());
				assert_eq!(iterator.next(), None);
			}

			#[test]
			fn loopback_max_value_properly_stops() {
				let net: NetAddr = "ffff:ffff:ffff:ffff:ffff:ffff:ffff:ffff/127"
					.parse()
					.unwrap();

				let mut iterator: AddressIterator<NetAddr, IpAddr> = net.iter();
				assert_eq!(
					iterator.next(),
					"ffff:ffff:ffff:ffff:ffff:ffff:ffff:fffe"
						.parse::<IpAddr>()
						.ok()
				);
				assert_eq!(
					iterator.next(),
					"ffff:ffff:ffff:ffff:ffff:ffff:ffff:ffff"
						.parse::<IpAddr>()
						.ok()
				);
				assert_eq!(iterator.next(), None);
			}
		}
	}
}
