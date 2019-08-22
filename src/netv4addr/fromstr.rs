use super::Netv4Addr;
use crate::traits::Mask;
use crate::NetAddrError;
use core::str::FromStr;
use std::net::Ipv4Addr;

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
