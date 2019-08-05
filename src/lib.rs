use core::cmp::Ordering;
use core::str::FromStr;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NetAddrError {
	ParseError(String),
}

impl From<std::net::AddrParseError> for NetAddrError {
	fn from(other: std::net::AddrParseError) -> Self {
		NetAddrError::ParseError(other.to_string())
	}
}

pub trait Broadcast {
	type Output;

	fn broadcast(&self) -> Self::Output;
}

impl Broadcast for Netv4Addr {
	type Output = Ipv4Addr;

	fn broadcast(&self) -> Ipv4Addr {
		let netmask: u32 = self.mask().clone().into();
		let network: u32 = self.addr().clone().into();
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
		let addr: u32 = self.addr().clone().into();
		let mask: u32 = self.mask().clone().into();
		let other_addr: u32 = other.addr().clone().into();
		let other_mask: u32 = other.mask().clone().into();

		let mask: u32 = match mask.cmp(&other_mask) {
			Ordering::Equal => mask << 1,
			Ordering::Less => mask,
			Ordering::Greater => other_mask,
		};

		if addr & mask == other_addr & mask {
			Some(Self::new(Ipv4Addr::from(addr & mask), Ipv4Addr::from(mask)))
		} else {
			None
		}
	}
}

impl Merge for Netv6Addr {
	type Output = Option<Self>;

	fn merge(&self, other: &Self) -> Self::Output {
		let addr: u128 = self.addr().clone().into();
		let mask: u128 = self.mask().clone().into();
		let other_addr: u128 = other.addr().clone().into();
		let other_mask: u128 = other.mask().clone().into();

		let mask: u128 = match mask.cmp(&other_mask) {
			Ordering::Equal => mask << 1,
			Ordering::Less => mask,
			Ordering::Greater => other_mask,
		};

		if addr & mask == other_addr & mask {
			Some(Self::new(Ipv6Addr::from(addr & mask), Ipv6Addr::from(mask)))
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
		Self: From<T>;
}

impl Contains for Netv4Addr {
	fn contains<T: Copy>(&self, other: &T) -> bool
	where
		Self: From<T>,
	{
		let other: Self = Self::from(*other);
		other.addr().mask(&self.mask()) == *self.addr()
	}
}

impl Contains for Netv6Addr {
	fn contains<T: Copy>(&self, other: &T) -> bool
	where
		Self: From<T>,
	{
		let other: Self = Self::from(*other);
		other.addr().mask(&self.mask()) == *self.addr()
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

impl From<IpAddr> for NetAddr {
	fn from(addr: IpAddr) -> Self {
		match addr {
			IpAddr::V4(addr) => NetAddr::V4(Netv4Addr::new(addr, Self::F32V4)),
			IpAddr::V6(addr) => NetAddr::V6(Netv6Addr::new(addr, Self::F32V6)),
		}
	}
}

impl From<Ipv4Addr> for NetAddr {
	fn from(addr: Ipv4Addr) -> Self {
		NetAddr::V4(Netv4Addr::new(addr, Self::F32V4))
	}
}

impl From<Ipv4Addr> for Netv4Addr {
	fn from(addr: Ipv4Addr) -> Self {
		Self::new(addr, NetAddr::F32V4)
	}
}

impl From<Ipv6Addr> for NetAddr {
	fn from(addr: Ipv6Addr) -> Self {
		NetAddr::V6(Netv6Addr::new(addr, Self::F32V6))
	}
}

impl From<Ipv6Addr> for Netv6Addr {
	fn from(addr: Ipv6Addr) -> Self {
		Self::new(addr, NetAddr::F32V6)
	}
}

impl FromStr for Netv4Addr {
	type Err = NetAddrError;

	/// Parse a `Netv4Addr` from a string
	///
	/// Often used implicitly, this implementation allows for a few formats to be given:
	/// - (Standard) CIDR format: `192.0.2.16/29`
	/// - Extended format (` `-delimited): `192.0.2.16 255.255.255.248`
	/// - Extended format (`/`-delimited): `192.0.2.16/255.255.255.248`
	///
	/// # Examples
	///
	/// ```rust
	/// # use netaddr2::Netv4Addr;
	/// let parsed: Netv4Addr = "192.0.2.16/29".parse().unwrap();
	/// let addr: std::net::Ipv4Addr = "192.0.2.16".parse().unwrap();
	/// let mask: std::net::Ipv4Addr = "255.255.255.248".parse().unwrap();
	/// assert_eq!(parsed, Netv4Addr::new(addr, mask));
	/// ```
	///
	/// ```rust
	/// # use netaddr2::Netv4Addr;
	/// let parsed: Netv4Addr = "192.0.2.16 255.255.255.248".parse().unwrap();
	/// let addr: std::net::Ipv4Addr = "192.0.2.16".parse().unwrap();
	/// let mask: std::net::Ipv4Addr = "255.255.255.248".parse().unwrap();
	/// assert_eq!(parsed, Netv4Addr::new(addr, mask));
	/// ```
	///
	/// ```rust
	/// # use netaddr2::Netv4Addr;
	/// let parsed: Netv4Addr = "192.0.2.16/255.255.255.248".parse().unwrap();
	/// let addr: std::net::Ipv4Addr = "192.0.2.16".parse().unwrap();
	/// let mask: std::net::Ipv4Addr = "255.255.255.248".parse().unwrap();
	/// assert_eq!(parsed, Netv4Addr::new(addr, mask));
	/// ```
	fn from_str(string: &str) -> Result<Self, NetAddrError> {
		let split: Vec<&str> = string.split(|c| c == '/' || c == ' ').collect();

		let lhs: &str = split[0];
		let rhs: &str = split
			.get(1)
			.ok_or_else(|| NetAddrError::ParseError("could not split provided input".to_string()))?;

		let address = lhs.parse::<Ipv4Addr>();
		let cidr = rhs.parse::<u32>();
		let right_addr = rhs.parse::<Ipv4Addr>();

		match (address, cidr, right_addr) {
			(Ok(addr), Ok(cidr), _) => {
				let mask: u32 = u32::max_value()
					^ match u32::max_value().checked_shr(cidr) {
						Some(k) => k,
						None => 0_u32,
					};

				let mask: Ipv4Addr = mask.into();

				Ok(Self::new(addr.mask(&mask), mask))
			}
			(Ok(addr), Err(_), Ok(mask)) => Ok(Self::new(addr.mask(&mask), mask)),
			(Ok(addr), Err(_), Err(_)) => Ok(Self::from(addr)),
			(Err(e), _, _) => Err(e.into()),
		}
	}
}

impl FromStr for Netv6Addr {
	type Err = NetAddrError;

	/// Parse a `Netv6Addr` from a string
	///
	/// Often used implicitly, this implementation allows for a few formats to be given:
	/// - (Standard) CIDR format: `2001:db8:dead:beef::1/64`
	/// - Extended format: `2001:db8:dead:beef::1 ffff:ffff:ffff:ffff::`
	/// - Extended format (with a `/` delimiter): `2001:db8:dead:beef::1/ffff:ffff:ffff:ffff::`
	///
	/// # Examples
	///
	/// ```rust
	/// # use netaddr2::Netv6Addr;
	/// let parsed: Netv6Addr = "2001:db8:dead:beef::1/32".parse().unwrap();
	/// let addr: std::net::Ipv6Addr = "2001:db8::0".parse().unwrap();
	/// let mask: std::net::Ipv6Addr = "ffff:ffff::0".parse().unwrap();
	/// assert_eq!(parsed, Netv6Addr::new(addr, mask))
	/// ```
	///
	/// ```rust
	/// # use netaddr2::Netv6Addr;
	/// let parsed: Netv6Addr = "2001:db8:dead:beef::1 ffff:ffff::".parse().unwrap();
	/// let addr: std::net::Ipv6Addr = "2001:db8::0".parse().unwrap();
	/// let mask: std::net::Ipv6Addr = "ffff:ffff::0".parse().unwrap();
	/// assert_eq!(parsed, Netv6Addr::new(addr, mask))
	/// ```
	///
	/// ```rust
	/// # use netaddr2::Netv6Addr;
	/// let parsed: Netv6Addr = "2001:db8:dead:beef::1/ffff:ffff::".parse().unwrap();
	/// let addr: std::net::Ipv6Addr = "2001:db8::0".parse().unwrap();
	/// let mask: std::net::Ipv6Addr = "ffff:ffff::0".parse().unwrap();
	/// assert_eq!(parsed, Netv6Addr::new(addr, mask))
	/// ```
	fn from_str(string: &str) -> Result<Self, NetAddrError> {
		let split: Vec<&str> = string.split(|c| c == '/' || c == ' ').collect();

		let lhs: &str = split[0];
		let rhs: &str = split
			.get(1)
			.ok_or_else(|| NetAddrError::ParseError("could not split provided input".to_string()))?;

		let address = lhs.parse::<Ipv6Addr>();
		let cidr = rhs.parse::<u32>();
		let right_addr = rhs.parse::<Ipv6Addr>();

		match (address, cidr, right_addr) {
			(Ok(addr), Ok(cidr), _) => {
				let mask: u128 = u128::max_value()
					^ match u128::max_value().checked_shr(cidr) {
						Some(k) => k,
						None => 0_u128,
					};

				let mask: Ipv6Addr = mask.into();

				Ok(Self::new(addr, mask))
			}
			(Ok(addr), Err(_), Ok(mask)) => Ok(Self::new(addr, mask)),
			(Ok(addr), Err(_), Err(_)) => Ok(Self::from(addr)),
			(Err(e), _, _) => Err(e.into()),
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

impl PartialOrd for Netv4Addr {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		match self.addr().partial_cmp(other.addr()) {
			Some(Ordering::Equal) => self.mask().partial_cmp(other.mask()),
			Some(ordering) => Some(ordering),
			None => None,
		}
	}
}

impl PartialOrd for Netv6Addr {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		match self.addr().partial_cmp(other.addr()) {
			Some(Ordering::Equal) => self.mask().partial_cmp(other.mask()),
			Some(ordering) => Some(ordering),
			None => None,
		}
	}
}

mod mask;
pub use mask::*;

mod netaddr;
pub use netaddr::*;

#[cfg(test)]
mod tests;
