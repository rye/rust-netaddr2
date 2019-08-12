use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

pub trait Mask {
	type Output;

	fn mask(&self, other: &Self) -> Self::Output;
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
