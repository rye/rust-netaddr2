use crate::netaddr_error::NetAddrError;
use crate::traits::Contains;
use crate::traits::Merge;
use crate::traits::Mask;
use core::cmp::Ordering;
use core::str::FromStr;
use std::net::Ipv6Addr;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Ord)]
pub struct Netv6Addr {
	mask: Ipv6Addr,
	addr: Ipv6Addr,
}

impl Netv6Addr {
	pub(crate) fn mask(&self) -> &Ipv6Addr {
		&self.mask
	}

	pub(crate) fn addr(&self) -> &Ipv6Addr {
		&self.addr
	}

	pub fn new(addr: Ipv6Addr, mask: Ipv6Addr) -> Self {
		let addr = addr.mask(&mask);
		Self { addr, mask }
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

impl PartialOrd for Netv6Addr {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		match self.addr().partial_cmp(other.addr()) {
			Some(Ordering::Equal) => self.mask().partial_cmp(other.mask()),
			Some(ordering) => Some(ordering),
			None => None,
		}
	}
}
