use core::cmp::Ordering;
use core::str::FromStr;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

/// A structure representing an IP network.
///
/// Internally using the built-in `std::net::IpAddr` structures, this is a
/// simple data structure that can be used in a variety of situations.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Ord)]
pub struct NetAddr {
	pub netmask: IpAddr,
	pub network: IpAddr,
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
/// ```should_panic
/// # use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
/// let addr: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
/// let mask: IpAddr = IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 255, 0, 0, 0));
/// netaddr2::mask(&addr, &mask);
/// ```
pub fn mask(addr: &IpAddr, mask: &IpAddr) -> IpAddr {
	match (addr, mask) {
		(IpAddr::V4(addr), IpAddr::V4(mask)) => {
			let (addr, mask): (u32, u32) = ((*addr).into(), (*mask).into());
			IpAddr::V4((addr & mask).into())
		}
		(IpAddr::V6(addr), IpAddr::V6(mask)) => {
			let (addr, mask): (u128, u128) = ((*addr).into(), (*mask).into());
			IpAddr::V6((addr & mask).into())
		}
		(_, _) => panic!("mismatched types"),
	}
}

impl NetAddr {
	pub fn contains(&self, other: &IpAddr) -> bool {
		match (self.netmask, self.network, other) {
			(IpAddr::V4(netmask), IpAddr::V4(network), IpAddr::V4(other)) => {
				let other: u32 = (*other).into();
				let mask: u32 = netmask.into();
				let network: u32 = network.into();

				(other & mask) == network
			}
			(IpAddr::V6(netmask), IpAddr::V6(network), IpAddr::V6(other)) => {
				let other: u128 = (*other).into();
				let mask: u128 = netmask.into();
				let network: u128 = network.into();

				(other & mask) == network
			}
			(_, _, _) => panic!("mismatched address types"),
		}
	}

	pub fn broadcast(&self) -> Option<IpAddr> {
		match (self.network, self.netmask) {
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
		match (self.network, self.netmask, other.network, other.netmask) {
			(
				IpAddr::V4(network),
				IpAddr::V4(netmask),
				IpAddr::V4(other_network),
				IpAddr::V4(other_netmask),
			) => {
				let network: u32 = network.into();
				let netmask: u32 = netmask.into();
				let other_network: u32 = other_network.into();
				let other_netmask: u32 = other_netmask.into();

				let netmask: u32 = match netmask.cmp(&other_netmask) {
					Ordering::Equal => netmask << 1,
					Ordering::Less => netmask,
					Ordering::Greater => other_netmask,
				};

				if network & netmask == other_network & netmask {
					Some(NetAddr {
						network: IpAddr::V4(Ipv4Addr::from(network & netmask)),
						netmask: IpAddr::V4(Ipv4Addr::from(netmask)),
					})
				} else {
					None
				}
			}
			(
				IpAddr::V6(_network),
				IpAddr::V6(_netmask),
				IpAddr::V6(_other_network),
				IpAddr::V6(_other_netmask),
			) => unimplemented!(),
			(_, _, _, _) => unimplemented!(),
		}
	}
}

impl From<IpAddr> for NetAddr {
	fn from(addr: IpAddr) -> Self {
		Self {
			network: addr,
			netmask: match addr {
				IpAddr::V4(_) => IpAddr::V4(Ipv4Addr::new(255, 255, 255, 255)),
				IpAddr::V6(_) => IpAddr::V6(Ipv6Addr::new(
					0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff,
				)),
			},
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

		match (as_u32, as_ipaddr) {
			(Ok(cidr_prefix_length), _) => match address {
				Ok(IpAddr::V4(address)) => {
					let mask: u32 = 0xff_ff_ff_ff_u32
						^ match 0xff_ff_ff_ff_u32.checked_shr(cidr_prefix_length) {
							Some(k) => k,
							None => 0_u32,
						};

					let netmask = IpAddr::V4(mask.into());

					let network = crate::mask(&address.into(), &netmask);

					Ok(Self { network, netmask })
				}
				Ok(IpAddr::V6(address)) => {
					let mask: u128 = 0xffff_ffff_ffff_ffff_ffff_ffff_ffff_ffff_u128
						^ match 0xffff_ffff_ffff_ffff_ffff_ffff_ffff_ffff_u128.checked_shr(cidr_prefix_length) {
							Some(k) => k,
							None => 0_u128,
						};

					let netmask = IpAddr::V6(mask.into());

					let network = crate::mask(&address.into(), &netmask);

					Ok(Self { network, netmask })
				}
				Err(e) => Err(e.into()),
			},
			(Err(_), Ok(netmask)) => match address {
				Ok(address) => {
					let network = crate::mask(&address, &netmask);

					Ok(Self { network, netmask })
				}
				Err(e) => Err(e.into()),
			},
			(Err(_), Err(e)) => Err(e.into()),
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
		match self.network.partial_cmp(&other.network) {
			Some(Ordering::Equal) => self.netmask.partial_cmp(&other.netmask),
			Some(ordering) => Some(ordering),
			None => None,
		}
	}
}

#[cfg(test)]
mod tests;
