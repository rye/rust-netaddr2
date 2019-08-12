use core::str::FromStr;
use std::net::{Ipv4Addr, Ipv6Addr};

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

impl From<Ipv4Addr> for NetAddr {
	fn from(addr: Ipv4Addr) -> Self {
		NetAddr::V4(Netv4Addr::from(addr))
	}
}

impl From<Ipv4Addr> for Netv4Addr {
	fn from(addr: Ipv4Addr) -> Self {
		Self::new(addr, Ipv4Addr::from(u32::max_value()))
	}
}

impl From<Ipv6Addr> for NetAddr {
	fn from(addr: Ipv6Addr) -> Self {
		NetAddr::V6(Netv6Addr::from(addr))
	}
}

impl From<Ipv6Addr> for Netv6Addr {
	fn from(addr: Ipv6Addr) -> Self {
		Self::new(addr, Ipv6Addr::from(u128::max_value()))
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

mod broadcast;
mod contains;
mod mask;
mod merge;
pub use broadcast::*;
pub use contains::*;
pub use mask::*;
pub use merge::*;

mod netaddr;
mod netv4addr;
mod netv6addr;
pub use netaddr::*;
pub use netv4addr::*;
pub use netv6addr::*;

mod netaddr_error;
pub use netaddr_error::*;

#[cfg(test)]
mod tests;
