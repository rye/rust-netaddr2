use crate::netaddr_error::NetAddrError;
use crate::traits::Broadcast;
use crate::traits::Contains;
use crate::traits::Merge;
use crate::traits::Mask;
use core::cmp::Ordering;
use core::str::FromStr;
use std::net::Ipv4Addr;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Ord)]
pub struct Netv4Addr {
	mask: Ipv4Addr,
	addr: Ipv4Addr,
}

impl Netv4Addr {
	pub(crate) fn mask(&self) -> &Ipv4Addr {
		&self.mask
	}

	pub(crate) fn addr(&self) -> &Ipv4Addr {
		&self.addr
	}

	pub fn new(addr: Ipv4Addr, mask: Ipv4Addr) -> Self {
		let addr = addr.mask(&mask);
		Self { addr, mask }
	}
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

impl Contains for Netv4Addr {
	fn contains<T: Copy>(&self, other: &T) -> bool
	where
		Self: From<T>,
	{
		let other: Self = Self::from(*other);
		other.addr().mask(&self.mask()) == *self.addr()
	}
}

impl From<Ipv4Addr> for Netv4Addr {
	fn from(addr: Ipv4Addr) -> Self {
		Self::new(addr, Ipv4Addr::from(u32::max_value()))
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

impl PartialOrd for Netv4Addr {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		match self.addr().partial_cmp(other.addr()) {
			Some(Ordering::Equal) => self.mask().partial_cmp(other.mask()),
			Some(ordering) => Some(ordering),
			None => None,
		}
	}
}
