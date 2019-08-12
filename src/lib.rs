use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

impl From<Ipv4Addr> for Netv4Addr {
	fn from(addr: Ipv4Addr) -> Self {
		Self::new(addr, Ipv4Addr::from(u32::max_value()))
	}
}

impl From<Ipv6Addr> for Netv6Addr {
	fn from(addr: Ipv6Addr) -> Self {
		Self::new(addr, Ipv6Addr::from(u128::max_value()))
	}
}

impl Mask for IpAddr {
	type Output = Result<Self, &'static str>;

	fn mask(&self, other: &Self) -> Self::Output {
		match (self, other) {
			// TODO convert to Self::V4, Self::V6 once stabilized in 1.37 (2019-08-15)
			(IpAddr::V4(a), IpAddr::V4(b)) => Ok(IpAddr::V4(a.mask(&b))),
			// TODO convert to Self::V4, Self::V6 once stabilized in 1.37 (2019-08-15)
			(IpAddr::V6(a), IpAddr::V6(b)) => Ok(IpAddr::V6(a.mask(&b))),
			(_, _) => Err("mismatched address types"),
		}
	}
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
