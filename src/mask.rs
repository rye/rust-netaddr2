use std::net::Ipv4Addr;
use std::net::Ipv6Addr;

pub trait Mask {
	type Output;

	fn mask(&self, other: &Self) -> Self::Output;
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
