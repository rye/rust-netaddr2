use crate::{NetAddr, Netv6Addr};
use std::net::Ipv6Addr;

impl From<Ipv6Addr> for NetAddr {
	fn from(addr: Ipv6Addr) -> Self {
		Self::V6(Netv6Addr::from(addr))
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::net::Ipv6Addr;

	#[test]
	fn uses_max_netmask() {
		let addr: Ipv6Addr = "2001:db8:dead:beef::42".parse().unwrap();
		let netaddr: NetAddr = NetAddr::from(addr);
		assert_eq!(
			netaddr,
			NetAddr::V6(Netv6Addr::new(addr, Ipv6Addr::from(u128::max_value())))
		);
	}
}
