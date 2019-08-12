use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

pub trait Mask {
	type Output;

	fn mask(&self, other: &Self) -> Self::Output;
}
