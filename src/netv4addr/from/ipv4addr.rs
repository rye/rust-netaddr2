use std::net::Ipv4Addr;

use crate::netv4addr::Netv4Addr;

impl From<Ipv4Addr> for Netv4Addr {
	fn from(addr: Ipv4Addr) -> Self {
		Self::new(addr, Ipv4Addr::from(u32::MAX))
	}
}

#[cfg(test)]
mod tests {
	use std::net::Ipv4Addr;

	use crate::netv4addr::Netv4Addr;

	#[test]
	fn uses_max_netmask() {
		let addr: Ipv4Addr = "192.0.2.42".parse().unwrap();
		let netaddr: Netv4Addr = Netv4Addr::from(addr);
		assert_eq!(netaddr, Netv4Addr::new(addr, Ipv4Addr::from(u32::MAX)));
	}
}
