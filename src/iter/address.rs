use crate::Contains;

use super::offset::Offset;

/// An iterator over a network's _contained addresses_.
pub struct AddressIterator<Network, Address>
where
	Network: Contains<Address> + From<Address>,
{
	net: Network,
	cur: Option<Address>,
}

impl<Network, Address> AddressIterator<Network, Address>
where
	Network: Contains<Address> + From<Address>,
{
	pub(crate) fn new(net: Network, cur: Option<Address>) -> Self {
		Self { net, cur }
	}
}

/// Implementation of the [`Iterator`] trait for [`AddressIterator`].
impl<Network, Address> Iterator for AddressIterator<Network, Address>
where
	Address: Copy + Offset<u32>,
	Network: Contains<Address> + From<Address>,
{
	type Item = Address;

	/// Produce the next item.
	///
	/// The current address is offset by `1_u32` using the [`Offset`] trait, which
	/// may produce `None`.
	fn next(&mut self) -> Option<Self::Item> {
		let cur: Option<Self::Item> = self.cur;
		let next: Option<Self::Item> = cur.and_then(|cur| cur.offset(1_u32));

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
				self.cur = None;
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
					cur: Some(self.addr()),
				}
			}
		}

		#[test]
		fn loopback_slash_32_produces_one_off() {
			let net: Netv4Addr = "127.0.16.0/32".parse().unwrap();

			let mut it: AddressIterator<Netv4Addr, Ipv4Addr> = net.iter();
			assert_eq!(it.next(), "127.0.16.0".parse::<Ipv4Addr>().ok());
			assert_eq!(it.next(), None);
		}

		#[test]
		fn loopback_slash_29_produces_one_off() {
			let net: Netv4Addr = "127.0.16.0/29".parse().unwrap();

			let mut it: AddressIterator<Netv4Addr, Ipv4Addr> = net.iter();
			assert_eq!(it.next(), "127.0.16.0".parse::<Ipv4Addr>().ok());
			assert_eq!(it.next(), "127.0.16.1".parse::<Ipv4Addr>().ok());
			assert_eq!(it.next(), "127.0.16.2".parse::<Ipv4Addr>().ok());
			assert_eq!(it.next(), "127.0.16.3".parse::<Ipv4Addr>().ok());
			assert_eq!(it.next(), "127.0.16.4".parse::<Ipv4Addr>().ok());
			assert_eq!(it.next(), "127.0.16.5".parse::<Ipv4Addr>().ok());
			assert_eq!(it.next(), "127.0.16.6".parse::<Ipv4Addr>().ok());
			assert_eq!(it.next(), "127.0.16.7".parse::<Ipv4Addr>().ok());
			assert_eq!(it.next(), None);
		}

		#[test]
		fn loopback_max_value_properly_stops() {
			let net: Netv4Addr = "255.255.255.255/31".parse().unwrap();

			let mut it: AddressIterator<Netv4Addr, Ipv4Addr> = net.iter();
			assert_eq!(it.next(), "255.255.255.254".parse::<Ipv4Addr>().ok());
			assert_eq!(it.next(), "255.255.255.255".parse::<Ipv4Addr>().ok());
			assert_eq!(it.next(), None);
		}
	}

	mod netaddr {
		use super::*;

		use crate::NetAddr;
		use std::net::IpAddr;

		mod v4 {
			use super::*;

			#[test]
			fn loopback_slash_32_produces_one_off() {
				let net: NetAddr = "127.0.16.0/32".parse().unwrap();

				let mut it: AddressIterator<NetAddr, IpAddr> = net.iter();
				assert_eq!(it.next(), "127.0.16.0".parse::<IpAddr>().ok());
				assert_eq!(it.next(), None);
			}

			#[test]
			fn loopback_slash_29_produces_all_ips_in_network() {
				let net: NetAddr = "127.0.16.0/29".parse().unwrap();

				let mut it: AddressIterator<NetAddr, IpAddr> = net.iter();
				assert_eq!(it.next(), "127.0.16.0".parse::<IpAddr>().ok());
				assert_eq!(it.next(), "127.0.16.1".parse::<IpAddr>().ok());
				assert_eq!(it.next(), "127.0.16.2".parse::<IpAddr>().ok());
				assert_eq!(it.next(), "127.0.16.3".parse::<IpAddr>().ok());
				assert_eq!(it.next(), "127.0.16.4".parse::<IpAddr>().ok());
				assert_eq!(it.next(), "127.0.16.5".parse::<IpAddr>().ok());
				assert_eq!(it.next(), "127.0.16.6".parse::<IpAddr>().ok());
				assert_eq!(it.next(), "127.0.16.7".parse::<IpAddr>().ok());
				assert_eq!(it.next(), None);
			}

			#[test]
			fn loopback_max_value_properly_stops() {
				let net: NetAddr = "255.255.255.255/31".parse().unwrap();

				let mut it: AddressIterator<NetAddr, IpAddr> = net.iter();
				assert_eq!(it.next(), "255.255.255.254".parse::<IpAddr>().ok());
				assert_eq!(it.next(), "255.255.255.255".parse::<IpAddr>().ok());
				assert_eq!(it.next(), None);
			}
		}

		mod v6 {
			use super::*;

			#[test]
			fn slash_128_produces_one_off() {
				let net: NetAddr = "2001:db8::1/128".parse().unwrap();

				let mut it: AddressIterator<NetAddr, IpAddr> = net.iter();
				assert_eq!(it.next(), "2001:db8::1".parse::<IpAddr>().ok());
				assert_eq!(it.next(), None);
			}

			#[test]
			fn slash_125_produces_all_ips_in_network() {
				let net: NetAddr = "2001:db8::1/125".parse().unwrap();

				let mut it: AddressIterator<NetAddr, IpAddr> = net.iter();
				assert_eq!(it.next(), "2001:db8::0".parse::<IpAddr>().ok());
				assert_eq!(it.next(), "2001:db8::1".parse::<IpAddr>().ok());
				assert_eq!(it.next(), "2001:db8::2".parse::<IpAddr>().ok());
				assert_eq!(it.next(), "2001:db8::3".parse::<IpAddr>().ok());
				assert_eq!(it.next(), "2001:db8::4".parse::<IpAddr>().ok());
				assert_eq!(it.next(), "2001:db8::5".parse::<IpAddr>().ok());
				assert_eq!(it.next(), "2001:db8::6".parse::<IpAddr>().ok());
				assert_eq!(it.next(), "2001:db8::7".parse::<IpAddr>().ok());
				assert_eq!(it.next(), None);
			}

			#[test]
			fn loopback_max_value_properly_stops() {
				let net: NetAddr = "ffff:ffff:ffff:ffff:ffff:ffff:ffff:ffff/127"
					.parse()
					.unwrap();

				let mut it: AddressIterator<NetAddr, IpAddr> = net.iter();
				assert_eq!(
					it.next(),
					"ffff:ffff:ffff:ffff:ffff:ffff:ffff:fffe"
						.parse::<IpAddr>()
						.ok()
				);
				assert_eq!(
					it.next(),
					"ffff:ffff:ffff:ffff:ffff:ffff:ffff:ffff"
						.parse::<IpAddr>()
						.ok()
				);
				assert_eq!(it.next(), None);
			}
		}
	}
}
