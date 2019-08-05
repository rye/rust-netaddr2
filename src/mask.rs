use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::net::Ipv6Addr;

pub trait Mask {
	type Output;

	fn mask(&self, other: &Self) -> Self::Output;
}

impl Mask for IpAddr {
	type Output = Result<Self, &'static str>;

	fn mask(&self, other: &Self) -> Self::Output {
		match (self, other) {
			(Self::V4(a), Self::V4(b)) => Ok(Self::V4(a.mask(&b))),
			(Self::V6(a), Self::V6(b)) => Ok(Self::V6(a.mask(&b))),
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
