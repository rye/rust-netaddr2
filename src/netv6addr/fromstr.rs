use crate::netaddr_error::NetAddrError;
use crate::netv6addr::Netv6Addr;
use core::str::FromStr;

use std::net::Ipv6Addr;

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

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn invalid_is_safe() {
		let _: Result<Netv6Addr, _> = "zoop".parse::<Netv6Addr>();
	}

	#[test]
	fn addr_only_returns_full_bitstring() {
		let net: Netv6Addr = "ff02::1/zoop".parse().unwrap();
		assert_eq!(net, "ff02::1/128".parse().unwrap());
	}

	#[test]
	fn non_addr_passes_out_error() {
		let result = "zoop".parse::<Netv6Addr>();
		assert_eq!(
			result,
			Err(NetAddrError::ParseError(
				"could not split provided input".to_string()
			))
		);
	}
}
