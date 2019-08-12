use core::str::FromStr;
use std::net::{Ipv4Addr, Ipv6Addr};

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
