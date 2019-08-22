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

	mod fromstr {
		use crate::{NetAddr, NetAddrError};

		#[test]
		fn invalid_is_safe() {
			let _: Result<NetAddr, _> = "zoop".parse::<NetAddr>();
		}

		#[test]
		fn addr_only() {
			let net: NetAddr = "127.0.0.1/zoop".parse().unwrap();
			assert_eq!(net, "127.0.0.1/32".parse().unwrap());
		}

		#[test]
		fn addr_no_mask_returns_full_bitstring() {
			let net: NetAddr = "127.0.0.1/zoop".parse().unwrap();
			assert_eq!(net, "127.0.0.1/32".parse().unwrap());
			let net: NetAddr = "ff02::1/zoop".parse().unwrap();
			assert_eq!(net, "ff02::1/128".parse().unwrap());
		}

		#[test]
		fn non_addr_passes_out_error() {
			let result = "zoop".parse::<NetAddr>();
			assert_eq!(
				result,
				Err(NetAddrError::ParseError(
					"could not split provided input".to_string()
				))
			);
		}

		mod v4 {
			use super::NetAddr;
			use std::net::{IpAddr, Ipv4Addr};

			#[test]
			fn correct_network() {
				let net: NetAddr = "192.0.2.0/32".parse().unwrap();
				assert_eq!(net.mask(), IpAddr::V4(Ipv4Addr::new(255, 255, 255, 255)));
				assert_eq!(net.addr(), IpAddr::V4(Ipv4Addr::new(192, 0, 2, 0)));
			}

			#[test]
			fn localhost_8() {
				let net: NetAddr = "127.0.0.1/8".parse().unwrap();
				assert_eq!(net.mask(), IpAddr::V4(Ipv4Addr::new(255, 0, 0, 0)));
				assert_eq!(net.addr(), IpAddr::V4(Ipv4Addr::new(127, 0, 0, 0)));
			}

			#[test]
			fn cidr_22() {
				let net: NetAddr = "192.168.16.1/22".parse().unwrap();
				assert_eq!(net.mask(), IpAddr::V4(Ipv4Addr::new(255, 255, 252, 0)));
				assert_eq!(net.addr(), IpAddr::V4(Ipv4Addr::new(192, 168, 16, 0)));
			}

			#[test]
			fn extended_localhost() {
				let net: NetAddr = "127.0.0.1 255.0.0.0".parse().unwrap();
				assert_eq!(net.mask(), IpAddr::V4(Ipv4Addr::new(255, 0, 0, 0)));
				assert_eq!(net.addr(), IpAddr::V4(Ipv4Addr::new(127, 0, 0, 0)));
			}

			#[test]
			fn slashed_localhost() {
				let net: NetAddr = "127.0.0.1/255.0.0.0".parse().unwrap();
				assert_eq!(net.mask(), IpAddr::V4(Ipv4Addr::new(255, 0, 0, 0)));
				assert_eq!(net.addr(), IpAddr::V4(Ipv4Addr::new(127, 0, 0, 0)));
			}
		}

		mod v6 {
			use super::NetAddr;
			use std::net::{IpAddr, Ipv6Addr};

			#[test]
			fn v6_cidr_8() {
				let net: NetAddr = "ff02::1/8".parse().unwrap();
				assert_eq!(
					net.mask(),
					IpAddr::V6(Ipv6Addr::new(0xff00, 0, 0, 0, 0, 0, 0, 0x0000))
				);
				assert_eq!(
					net.addr(),
					IpAddr::V6(Ipv6Addr::new(0xff00, 0, 0, 0, 0, 0, 0, 0x0000))
				);
			}

			#[test]
			fn v6_cidr_128() {
				let net: NetAddr = "ff02::1/128".parse().unwrap();
				assert_eq!(
					net.mask(),
					IpAddr::V6(Ipv6Addr::new(
						0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff
					))
				);
				assert_eq!(
					net.addr(),
					IpAddr::V6(Ipv6Addr::new(0xff02, 0, 0, 0, 0, 0, 0, 0x0001))
				);
			}

			#[test]
			fn v6_extended() {
				let net: NetAddr = "ff02::1 ffff::0".parse().unwrap();
				assert_eq!(
					net.mask(),
					IpAddr::V6(Ipv6Addr::new(0xffff, 0, 0, 0, 0, 0, 0, 0))
				);
				assert_eq!(
					net.addr(),
					IpAddr::V6(Ipv6Addr::new(0xff02, 0, 0, 0, 0, 0, 0, 0))
				);
			}

			#[test]
			fn v6_slashed() {
				let net: NetAddr = "ff02::1/128".parse().unwrap();
				assert_eq!(
					net.mask(),
					IpAddr::V6(Ipv6Addr::new(
						0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff
					))
				);
				assert_eq!(
					net.addr(),
					IpAddr::V6(Ipv6Addr::new(0xff02, 0, 0, 0, 0, 0, 0, 0x0001))
				);
			}
		}
	}

	mod partialord {
		use super::NetAddr;
		use std::cmp::Ordering;

		mod v4 {
			use super::*;

			#[test]
			fn different_networks() {
				let a: NetAddr = "1.0.0.0/8".parse().unwrap();
				let b: NetAddr = "2.0.0.0/8".parse().unwrap();

				assert_eq!(a.partial_cmp(&b), Some(Ordering::Less))
			}

			#[test]
			fn different_netmasks() {
				let a: NetAddr = "1.0.0.0/7".parse().unwrap();
				let b: NetAddr = "1.0.0.0/8".parse().unwrap();

				assert_eq!(a.partial_cmp(&b), Some(Ordering::Less))
			}

			#[test]
			fn different() {
				let a: NetAddr = "1.0.0.0/8".parse().unwrap();
				let b: NetAddr = "0.0.0.0/24".parse().unwrap();

				assert_eq!(a.partial_cmp(&b), Some(Ordering::Greater))
			}

			#[test]
			fn equal() {
				let a: NetAddr = "1.0.0.0/8".parse().unwrap();
				let b: NetAddr = "1.0.0.0/8".parse().unwrap();

				assert_eq!(a.partial_cmp(&b), Some(Ordering::Equal))
			}
		}

		mod v6 {
			use super::*;

			#[test]
			fn different_networks() {
				let a: NetAddr = "2001:db8:0:0::0/64".parse().unwrap();
				let b: NetAddr = "2001:db8:0:1::0/64".parse().unwrap();

				assert_eq!(a.partial_cmp(&b), Some(Ordering::Less))
			}

			#[test]
			fn different_netmasks() {
				let a: NetAddr = "2001:db8:0:0::0/63".parse().unwrap();
				let b: NetAddr = "2001:db8:0:0::0/64".parse().unwrap();

				assert_eq!(a.partial_cmp(&b), Some(Ordering::Less))
			}

			#[test]
			fn different() {
				let a: NetAddr = "ff02::1/16".parse().unwrap();
				let b: NetAddr = "2001:db8:0:1::0/64".parse().unwrap();

				assert_eq!(a.partial_cmp(&b), Some(Ordering::Greater))
			}

			#[test]
			fn equal() {
				let a: NetAddr = "2001:db8:dead:beef::0/64".parse().unwrap();
				let b: NetAddr = "2001:db8:dead:beef::0/64".parse().unwrap();

				assert_eq!(a.partial_cmp(&b), Some(Ordering::Equal))
			}
		}
	}
}
