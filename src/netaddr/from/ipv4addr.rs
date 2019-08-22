use crate::{NetAddr, Netv4Addr};
use std::net::Ipv4Addr;

impl From<Ipv4Addr> for NetAddr {
	fn from(addr: Ipv4Addr) -> Self {
		Self::V4(Netv4Addr::from(addr))
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn uses_max_netmask() {
		let addr: Ipv4Addr = "192.0.2.42".parse().unwrap();
		let netaddr: NetAddr = NetAddr::from(addr);
		assert_eq!(
			netaddr,
			NetAddr::V4(Netv4Addr::new(addr, Ipv4Addr::from(u32::max_value())))
		);
	}
}
