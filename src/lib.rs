use core::cmp::Ordering;
use core::str::FromStr;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

/// A structure representing an IP network.
///
/// Internally using the built-in `std::net::IpAddr` structures, this is a
/// simple data structure that can be used in a variety of situations.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Ord)]
pub enum NetAddr {
	V4 {
		netmask: Ipv4Addr,
		network: Ipv4Addr,
	},
	V6 {
		netmask: Ipv6Addr,
		network: Ipv6Addr,
	},
}

impl NetAddr {
	pub fn netmask(&self) -> IpAddr {
		match self {
			NetAddr::V4 { netmask, .. } => IpAddr::V4(*netmask),
			NetAddr::V6 { netmask, .. } => IpAddr::V6(*netmask),
		}
	}

	pub fn network(&self) -> IpAddr {
		match self {
			NetAddr::V4 { network, .. } => IpAddr::V4(*network),
			NetAddr::V6 { network, .. } => IpAddr::V6(*network),
		}
	}
}

#[derive(Debug)]
pub enum NetAddrError {
	ParseError(String),
}

impl std::convert::From<std::net::AddrParseError> for NetAddrError {
	fn from(other: std::net::AddrParseError) -> Self {
		NetAddrError::ParseError(other.to_string())
	}
}

/// Mask the given referenced `addr` with the given `mask`, returning a new
/// `IpAddr`.
///
/// Both `addr` and `mask` must be of the same `enum` variant for the
/// operation to succeed.
///
/// # Panics
///
/// This function will panic if the provided `addr` and `mask` are not of the
/// same enum variant.
///
/// ```
/// # use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
/// let addr = Ipv4Addr::new(127, 0, 0, 1);
/// let mask = Ipv4Addr::new(255, 0, 0, 0);
/// netaddr2::mask::<Ipv4Addr, u32>(&addr, &mask);
/// ```
pub fn mask<T, U>(addr: &T, mask: &T) -> T
where
	U: std::convert::From<T>,
	U: std::ops::BitAnd<U, Output = U>,
	T: std::convert::From<U>,
	T: Copy,
{
	let (addr, mask): (U, U) = ((*addr).into(), (*mask).into());
	T::from((addr & mask).into())
}

impl NetAddr {
	const F32: u32 = u32::max_value();
	const F128: u128 = u128::max_value();
	const F32V4: Ipv4Addr = Ipv4Addr::new(255, 255, 255, 255);
	const F32V6: Ipv6Addr = Ipv6Addr::new(
		0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff,
	);

	pub fn contains<T>(&self, other: &T) -> bool
	where
		T: Copy,
		NetAddr: std::convert::From<T>,
	{
		match (*self, NetAddr::from(*other)) {
			(
				NetAddr::V4 {
					network: net,
					netmask: mask,
				},
				NetAddr::V4 { network: onet, .. },
			) => {
				let onet: u32 = onet.into();
				let net: u32 = net.into();
				let mask: u32 = mask.into();

				(onet & mask) == net
			}
			(
				NetAddr::V6 {
					network: net,
					netmask: mask,
				},
				NetAddr::V6 { network: onet, .. },
			) => {
				let onet: u128 = onet.into();
				let net: u128 = net.into();
				let mask: u128 = mask.into();

				(onet & mask) == net
			}
			(_, _) => panic!("mismatched types"),
		}
	}

	pub fn broadcast(&self) -> Option<IpAddr> {
		match (self.network(), self.netmask()) {
			(IpAddr::V4(network), IpAddr::V4(netmask)) => {
				let netmask: u32 = netmask.into();
				let network: u32 = network.into();
				let broadcast: u32 = network | !netmask;
				Some(IpAddr::V4(broadcast.into()))
			}
			(_, _) => None,
		}
	}

	pub fn merge(&self, other: &NetAddr) -> Option<NetAddr> {
		match (
			self.network(),
			self.netmask(),
			other.network(),
			other.netmask(),
		) {
			(IpAddr::V4(net), IpAddr::V4(mask), IpAddr::V4(other_net), IpAddr::V4(other_mask)) => {
				let net: u32 = net.into();
				let mask: u32 = mask.into();
				let other_net: u32 = other_net.into();
				let other_mask: u32 = other_mask.into();

				let mask: u32 = match mask.cmp(&other_mask) {
					Ordering::Equal => mask << 1,
					Ordering::Less => mask,
					Ordering::Greater => other_mask,
				};

				if net & mask == other_net & mask {
					Some(NetAddr::V4 {
						network: Ipv4Addr::from(net & mask),
						netmask: Ipv4Addr::from(mask),
					})
				} else {
					None
				}
			}
			(IpAddr::V6(_net), IpAddr::V6(_mask), IpAddr::V6(_other_net), IpAddr::V6(_other_mask)) => unimplemented!(),
			(_, _, _, _) => unimplemented!(),
		}
	}
}

impl From<IpAddr> for NetAddr {
	fn from(addr: IpAddr) -> Self {
		match addr {
			IpAddr::V4(addr) => NetAddr::V4 {
				network: addr,
				netmask: NetAddr::F32V4,
			},
			IpAddr::V6(addr) => NetAddr::V6 {
				network: addr,
				netmask: NetAddr::F32V6,
			},
		}
	}
}

impl From<Ipv4Addr> for NetAddr {
	fn from(addr: Ipv4Addr) -> Self {
		NetAddr::V4 {
			network: addr,
			netmask: NetAddr::F32V4,
		}
	}
}

impl From<Ipv6Addr> for NetAddr {
	fn from(addr: Ipv6Addr) -> Self {
		NetAddr::V6 {
			network: addr,
			netmask: NetAddr::F32V6,
		}
	}
}

impl FromStr for NetAddr {
	type Err = NetAddrError;

	fn from_str(string: &str) -> Result<Self, NetAddrError> {
		let split: Vec<&str> = string.split(|c| c == '/' || c == ' ').collect();

		let lhs: &str = split[0];
		let rhs: &str = split
			.get(1)
			.ok_or_else(|| NetAddrError::ParseError("could not split provided input".to_string()))?;

		let address = lhs.parse::<IpAddr>();
		let as_u32 = rhs.parse::<u32>();
		let as_ipaddr = rhs.parse::<IpAddr>();

		match (address, as_u32, as_ipaddr) {
			(Ok(IpAddr::V4(address)), Ok(cidr_prefix_length), _) => {
				let mask: u32 = NetAddr::F32
					^ match NetAddr::F32.checked_shr(cidr_prefix_length) {
						Some(k) => k,
						None => 0_u32,
					};

				let netmask: Ipv4Addr = mask.into();

				let network: Ipv4Addr = crate::mask::<Ipv4Addr, u32>(&address, &netmask);

				Ok(NetAddr::V4 {
					network: network,
					netmask: netmask,
				})
			}
			(Ok(IpAddr::V6(address)), Ok(cidr_prefix_length), _) => {
				let mask: u128 = NetAddr::F128
					^ match NetAddr::F128.checked_shr(cidr_prefix_length) {
						Some(k) => k,
						None => 0_u128,
					};

				let netmask: Ipv6Addr = mask.into();

				let network = crate::mask::<Ipv6Addr, u128>(&address, &netmask);

				Ok(NetAddr::V6 { network, netmask })
			}
			(Ok(IpAddr::V4(address)), Err(_), Ok(IpAddr::V4(netmask))) => {
				let network = crate::mask::<Ipv4Addr, u32>(&address, &netmask);

				Ok(NetAddr::V4 { network, netmask })
			}
			(Ok(IpAddr::V6(address)), Err(_), Ok(IpAddr::V6(netmask))) => {
				let network = crate::mask::<Ipv6Addr, u128>(&address, &netmask);

				Ok(NetAddr::V6 { network, netmask })
			}
			(Ok(addr), Err(_), Err(_)) => Ok(NetAddr::from(addr)),
			(Err(e), _, _) => Err(e.into()),
			(Ok(_), Err(_), Ok(_)) => Err(NetAddrError::ParseError(
				"mismatched types of network/netmask".to_string(),
			)),
		}
	}
}

impl PartialOrd for NetAddr {
	/// Ordinalize two `NetAddr`s.
	///
	/// Two `NetAddr`s are first compared by network address, or if their network
	/// address is the same, instead by netmask.  Two `NetAddr`s are said to be
	/// equal if both their network address and netmask are the same.
	///
	/// # Examples
	///
	/// In this example, two networks of the same netmask but unequal network
	/// addresses are compared.
	///
	/// ```
	/// let a: netaddr2::NetAddr = "1.1.1.1/32".parse().unwrap();
	/// let b: netaddr2::NetAddr = "2.2.2.2/32".parse().unwrap();
	/// assert_eq!(a.partial_cmp(&b), Some(std::cmp::Ordering::Less));
	/// // or, more concisely:
	/// assert!(a < b);
	/// ```
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		match (self, other) {
			(
				NetAddr::V4 {
					network: net,
					netmask: mask,
				},
				NetAddr::V4 {
					network: other_net,
					netmask: other_mask,
				},
			) => match net.partial_cmp(other_net) {
				Some(Ordering::Equal) => mask.partial_cmp(other_mask),
				Some(ordering) => Some(ordering),
				None => None,
			},
			(
				NetAddr::V6 {
					network: net,
					netmask: mask,
				},
				NetAddr::V6 {
					network: other_net,
					netmask: other_mask,
				},
			) => match net.partial_cmp(other_net) {
				Some(Ordering::Equal) => mask.partial_cmp(other_mask),
				Some(ordering) => Some(ordering),
				None => None,
			},
			(_, _) => unimplemented!(),
		}
	}
}

#[cfg(test)]
mod tests;
