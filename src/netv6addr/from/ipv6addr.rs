use crate::netv6addr::Netv6Addr;
use std::net::Ipv6Addr;

impl From<Ipv6Addr> for Netv6Addr {
	fn from(addr: Ipv6Addr) -> Self {
		Self::new(addr, Ipv6Addr::from(u128::max_value()))
	}
}
