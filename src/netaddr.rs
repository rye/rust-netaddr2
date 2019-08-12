use crate::netaddr_error::NetAddrError;
use crate::netv4addr::Netv4Addr;
use crate::netv6addr::Netv6Addr;
use core::str::FromStr;
use std::net::IpAddr;

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
