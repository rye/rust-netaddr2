use crate::netaddr_error::NetAddrError;
use crate::netv4addr::Netv4Addr;
use crate::netv6addr::Netv6Addr;
use crate::traits::Broadcast;
use crate::traits::Contains;
use crate::traits::Merge;
use core::str::FromStr;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

/// A structure representing an IP network.
///
/// Internally using the built-in `std::net::IpAddr` structures, this is a
/// simple data structure that can be used in a variety of situations.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum NetAddr {
	V4(Netv4Addr),
	V6(Netv6Addr),
}

impl NetAddr {
	pub fn mask(&self) -> IpAddr {
		match self {
			Self::V4(v4) => IpAddr::V4(*v4.mask()),
			Self::V6(v6) => IpAddr::V6(*v6.mask()),
		}
	}

	pub fn addr(&self) -> IpAddr {
		match self {
			Self::V4(v4) => IpAddr::V4(*v4.addr()),
			Self::V6(v6) => IpAddr::V6(*v6.addr()),
		}
	}
}

impl Broadcast for NetAddr {
	type Output = Option<IpAddr>;

	fn broadcast(&self) -> Self::Output {
		match self {
			Self::V4(netaddr) => Some(IpAddr::from(netaddr.broadcast())),
			_ => None,
		}
	}
}

impl Contains for NetAddr {
	fn contains<T: Copy>(&self, other: &T) -> bool
	where
		Self: From<T>,
	{
		let other: Self = Self::from(*other);
		match (self, other) {
			(Self::V4(netaddr), Self::V4(other)) => netaddr.contains(&other),
			(Self::V6(netaddr), Self::V6(other)) => netaddr.contains(&other),
			(_, _) => false,
		}
	}
}

impl FromStr for NetAddr {
	type Err = NetAddrError;

	fn from_str(string: &str) -> Result<Self, NetAddrError> {
		let as_v4: Result<Netv4Addr, NetAddrError> = string.parse::<Netv4Addr>();
		let as_v6: Result<Netv6Addr, NetAddrError> = string.parse::<Netv6Addr>();

		match (as_v4, as_v6) {
			(Ok(v4), _) => Ok(Self::V4(v4)),
			(_, Ok(v6)) => Ok(Self::V6(v6)),
			(Err(_e4), Err(e6)) => Err(e6),
		}
	}
}

impl From<IpAddr> for NetAddr {
	fn from(addr: IpAddr) -> Self {
		match addr {
			IpAddr::V4(addr) => Self::from(addr),
			IpAddr::V6(addr) => Self::from(addr),
		}
	}
}

impl From<Ipv4Addr> for NetAddr {
	fn from(addr: Ipv4Addr) -> Self {
		Self::V4(Netv4Addr::from(addr))
	}
}

impl From<Ipv6Addr> for NetAddr {
	fn from(addr: Ipv6Addr) -> Self {
		Self::V6(Netv6Addr::from(addr))
	}
}

impl From<Netv4Addr> for NetAddr {
	fn from(netaddr: Netv4Addr) -> Self {
		Self::V4(netaddr)
	}
}

impl From<Netv6Addr> for NetAddr {
	fn from(netaddr: Netv6Addr) -> Self {
		Self::V6(netaddr)
	}
}

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
	use super::{NetAddr, Netv4Addr, Netv6Addr};

	mod from_ipaddr {
		use super::*;
		use std::net::IpAddr;

		mod v4 {
			use super::*;
			use std::net::Ipv4Addr;

			#[test]
			fn uses_max_netmask() {
				let addr: IpAddr = "192.0.2.42".parse().unwrap();
				let netaddr: NetAddr = NetAddr::from(addr);

				assert_eq!(
					netaddr,
					NetAddr::V4(Netv4Addr::new(
						Ipv4Addr::new(192, 0, 2, 42),
						Ipv4Addr::from(u32::max_value())
					))
				);
			}
		}

		mod v6 {
			use super::*;
			use std::net::Ipv6Addr;

			#[test]
			fn uses_max_netmask() {
				let addr: IpAddr = "2001:db8:dead:beef::42".parse().unwrap();
				let netaddr: NetAddr = NetAddr::from(addr);

				assert_eq!(
					netaddr,
					NetAddr::V6(Netv6Addr::new(
						Ipv6Addr::new(0x2001, 0xdb8, 0xdead, 0xbeef, 0, 0, 0, 0x0042),
						Ipv6Addr::from(u128::max_value())
					))
				);
			}
		}
	}

	mod from_ipv4addr {
		use super::*;
		use std::net::Ipv4Addr;

		#[test]
		fn uses_max_netmask() {
			let addr: Ipv4Addr = "192.0.2.42".parse().unwrap();
			let netaddr: NetAddr = NetAddr::from(addr);
			assert_eq!(
				netaddr,
				NetAddr::V4(Netv4Addr::new(addr, Ipv4Addr::from(u32::max_value())))
			);
		}
	}

	mod from_ipv6addr {
		use super::*;
		use std::net::Ipv6Addr;

		#[test]
		fn uses_max_netmask() {
			let addr: Ipv6Addr = "2001:db8:dead:beef::42".parse().unwrap();
			let netaddr: NetAddr = NetAddr::from(addr);
			assert_eq!(
				netaddr,
				NetAddr::V6(Netv6Addr::new(addr, Ipv6Addr::from(u128::max_value())))
			);
		}
	}
}
