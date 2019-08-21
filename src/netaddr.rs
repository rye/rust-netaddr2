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
			NetAddr::V4(v4) => IpAddr::V4(*v4.mask()),
			NetAddr::V6(v6) => IpAddr::V6(*v6.mask()),
		}
	}

	pub fn addr(&self) -> IpAddr {
		match self {
			NetAddr::V4(v4) => IpAddr::V4(*v4.addr()),
			NetAddr::V6(v6) => IpAddr::V6(*v6.addr()),
		}
	}
}

impl Broadcast for NetAddr {
	type Output = Option<IpAddr>;

	fn broadcast(&self) -> Self::Output {
		match self {
			NetAddr::V4(netaddr) => Some(IpAddr::from(netaddr.broadcast())),
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
			(NetAddr::V4(netaddr), NetAddr::V4(other)) => netaddr.contains(&other),
			(NetAddr::V6(netaddr), NetAddr::V6(other)) => netaddr.contains(&other),
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
			(Ok(v4), _) => Ok(NetAddr::V4(v4)),
			(_, Ok(v6)) => Ok(NetAddr::V6(v6)),
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
		NetAddr::V4(Netv4Addr::from(addr))
	}
}

impl From<Ipv6Addr> for NetAddr {
	fn from(addr: Ipv6Addr) -> Self {
		NetAddr::V6(Netv6Addr::from(addr))
	}
}

impl From<Netv4Addr> for NetAddr {
	fn from(netaddr: Netv4Addr) -> Self {
		NetAddr::V4(netaddr)
	}
}

impl From<Netv6Addr> for NetAddr {
	fn from(netaddr: Netv6Addr) -> Self {
		NetAddr::V6(netaddr)
	}
}

impl Merge for NetAddr {
	type Output = Option<Self>;

	fn merge(&self, other: &Self) -> Self::Output {
		match (self, other) {
			(NetAddr::V4(a), NetAddr::V4(b)) => a.merge(b).map(|netvxaddr: Netv4Addr| netvxaddr.into()),
			(NetAddr::V6(a), NetAddr::V6(b)) => a.merge(b).map(|netvxaddr: Netv6Addr| netvxaddr.into()),
			(_, _) => unimplemented!(),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn is_broadcast() {
		fn is_broadcast<T: Broadcast>() {};
		is_broadcast::<NetAddr>();
	}

	#[test]
	fn is_contains() {
		fn is_contains<T: Contains>() {};
		is_contains::<NetAddr>();
	}

	#[test]
	fn is_fromstr() {
		fn is_fromstr<T: FromStr>() {};
		is_fromstr::<NetAddr>();
	}

	#[test]
	fn is_from_ipaddr() {
		fn is_from_ipaddr<T: From<IpAddr>>() {};
		is_from_ipaddr::<NetAddr>();
	}

	#[test]
	fn is_from_ipv4addr() {
		fn is_from_ipv4addr<T: From<Ipv4Addr>>() {};
		is_from_ipv4addr::<NetAddr>();
	}

	#[test]
	fn is_from_ipv6addr() {
		fn is_from_ipv6addr<T: From<Ipv6Addr>>() {};
		is_from_ipv6addr::<NetAddr>();
	}

	mod from_ipv4addr {
		use super::*;

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

	#[test]
	fn is_from_netv4addr() {
		fn is_from_netv4addr<T: From<Netv4Addr>>() {};
		is_from_netv4addr::<NetAddr>();
	}

	#[test]
	fn is_from_netv6addr() {
		fn is_from_netv6addr<T: From<Netv6Addr>>() {};
		is_from_netv6addr::<NetAddr>();
	}
}
