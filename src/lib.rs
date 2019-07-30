use core::cmp::Ordering;
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

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Ord)]
pub struct Netv4Addr {
	mask: Ipv4Addr,
	addr: Ipv4Addr,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Ord)]
pub struct Netv6Addr {
	mask: Ipv6Addr,
	addr: Ipv6Addr,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NetAddrError {
	ParseError(String),
}

impl NetAddr {
	fn netmask(&self) -> IpAddr {
		match self {
			Self::V4(netaddr) => IpAddr::V4(netaddr.mask),
			Self::V6(netaddr) => IpAddr::V6(netaddr.mask),
		}
	}

	fn network(&self) -> IpAddr {
		match self {
			Self::V4(netaddr) => IpAddr::V4(netaddr.addr),
			Self::V6(netaddr) => IpAddr::V6(netaddr.addr),
		}
	}
}

impl std::convert::From<std::net::AddrParseError> for NetAddrError {
	fn from(other: std::net::AddrParseError) -> Self {
		NetAddrError::ParseError(other.to_string())
	}
}

pub trait Mask {
	type Output;

	fn mask(&self, other: &Self) -> Self;
}

impl Mask for Ipv4Addr {
	type Output = Self;

	fn mask(&self, other: &Self) -> Self::Output {
		Self::Output::from((u32::from(*self)) & (u32::from(*other)))
	}
}

impl Mask for Ipv6Addr {
	type Output = Self;

	fn mask(&self, other: &Self) -> Self::Output {
		Self::Output::from((u128::from(*self)) & (u128::from(*other)))
	}
}

pub trait Broadcast {
	type Output;

	fn broadcast(&self) -> Self::Output;
}

impl Broadcast for Netv4Addr {
	type Output = Ipv4Addr;

	fn broadcast(&self) -> Ipv4Addr {
		let netmask: u32 = self.mask.into();
		let network: u32 = self.addr.into();
		let broadcast: u32 = network | !netmask;
		broadcast.into()
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

trait Merge {
	type Output;

	fn merge(&self, other: &Self) -> Self::Output;
}

impl Merge for Netv4Addr {
	type Output = Option<Self>;

	fn merge(&self, other: &Self) -> Self::Output {
		let addr: u32 = self.addr.into();
		let mask: u32 = self.mask.into();
		let other_addr: u32 = other.addr.into();
		let other_mask: u32 = other.mask.into();

		let mask: u32 = match mask.cmp(&other_mask) {
			Ordering::Equal => mask << 1,
			Ordering::Less => mask,
			Ordering::Greater => other_mask,
		};

		if addr & mask == other_addr & mask {
			Some(Self {
				addr: Ipv4Addr::from(addr & mask),
				mask: Ipv4Addr::from(mask),
			})
		} else {
			None
		}
	}
}

impl Merge for Netv6Addr {
	type Output = Option<Self>;

	fn merge(&self, other: &Self) -> Self::Output {
		let addr: u128 = self.addr.into();
		let mask: u128 = self.mask.into();
		let other_addr: u128 = other.addr.into();
		let other_mask: u128 = other.mask.into();

		let mask: u128 = match mask.cmp(&other_mask) {
			Ordering::Equal => mask << 1,
			Ordering::Less => mask,
			Ordering::Greater => other_mask,
		};

		if addr & mask == other_addr & mask {
			Some(Self {
				addr: Ipv6Addr::from(addr & mask),
				mask: Ipv6Addr::from(mask),
			})
		} else {
			None
		}
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

pub trait Contains {
	fn contains<T: Copy>(&self, other: &T) -> bool
	where
		Self: std::convert::From<T>;
}

impl Contains for Netv4Addr {
	fn contains<T: Copy>(&self, other: &T) -> bool
	where
		Self: std::convert::From<T>
	{
		let other: Self = Self::from(*other);
		other.addr.mask(&self.mask) == self.addr
	}
}

impl Contains for Netv6Addr {
	fn contains<T: Copy>(&self, other: &T) -> bool
	where
		Self: std::convert::From<T>
	{
		let other: Self = Self::from(*other);
		other.addr.mask(&self.mask) == self.addr
	}
}

impl Contains for NetAddr {
	fn contains<T: Copy>(&self, other: &T) -> bool
	where
		Self: std::convert::From<T>
	{
		let other: Self = Self::from(*other);
		match (self, other) {
			(NetAddr::V4(netaddr), NetAddr::V4(other)) => netaddr.contains(&other),
			(NetAddr::V6(netaddr), NetAddr::V6(other)) => netaddr.contains(&other),
			(_, _) => false
		}
	}
}


impl NetAddr {
	const F32: u32 = u32::max_value();
	const F128: u128 = u128::max_value();
	const F32V4: Ipv4Addr = Ipv4Addr::new(255, 255, 255, 255);
	const F32V6: Ipv6Addr = Ipv6Addr::new(
		0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff,
	);
}

impl From<IpAddr> for NetAddr {
	fn from(addr: IpAddr) -> Self {
		match addr {
			IpAddr::V4(addr) => NetAddr::V4(Netv4Addr {
				addr,
				mask: Self::F32V4,
			}),
			IpAddr::V6(addr) => NetAddr::V6(Netv6Addr {
				addr,
				mask: Self::F32V6,
			}),
		}
	}
}

impl From<Ipv4Addr> for NetAddr {
	fn from(addr: Ipv4Addr) -> Self {
		NetAddr::V4(Netv4Addr {
			addr,
			mask: Self::F32V4,
		})
	}
}

impl From<Ipv6Addr> for NetAddr {
	fn from(addr: Ipv6Addr) -> Self {
		NetAddr::V6(Netv6Addr {
			addr,
			mask: Self::F32V6,
		})
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
				let mask: u32 = Self::F32
					^ match Self::F32.checked_shr(cidr_prefix_length) {
						Some(k) => k,
						None => 0_u32,
					};

				let mask: Ipv4Addr = mask.into();

				let addr: Ipv4Addr = address.mask(&mask);

				Ok(NetAddr::V4(Netv4Addr { addr, mask }))
			}
			(Ok(IpAddr::V6(address)), Ok(cidr_prefix_length), _) => {
				let mask: u128 = Self::F128
					^ match Self::F128.checked_shr(cidr_prefix_length) {
						Some(k) => k,
						None => 0_u128,
					};

				let mask: Ipv6Addr = mask.into();

				let addr = address.mask(&mask);

				Ok(NetAddr::V6(Netv6Addr { addr, mask }))
			}
			(Ok(IpAddr::V4(address)), Err(_), Ok(IpAddr::V4(mask))) => {
				let addr = address.mask(&mask);

				Ok(NetAddr::V4(Netv4Addr { addr, mask }))
			}
			(Ok(IpAddr::V6(address)), Err(_), Ok(IpAddr::V6(mask))) => {
				let addr = address.mask(&mask);

				Ok(NetAddr::V6(Netv6Addr { addr, mask }))
			}
			(Ok(addr), Err(_), Err(_)) => Ok(Self::from(addr)),
			(Err(e), _, _) => Err(e.into()),
			(Ok(_), Err(_), Ok(_)) => Err(NetAddrError::ParseError(
				"mismatched types of network/netmask".to_string(),
			)),
		}
	}
}

impl PartialOrd for Netv4Addr {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		match self.addr.partial_cmp(&other.addr) {
			Some(Ordering::Equal) => self.mask.partial_cmp(&other.mask),
			Some(ordering) => Some(ordering),
			None => None,
		}
	}
}

impl PartialOrd for Netv6Addr {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		match self.addr.partial_cmp(&other.addr) {
			Some(Ordering::Equal) => self.mask.partial_cmp(&other.mask),
			Some(ordering) => Some(ordering),
			None => None,
		}
	}
}

#[cfg(test)]
mod tests;
