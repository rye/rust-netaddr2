use super::Netv4Addr;
use std::net::Ipv4Addr;

impl From<Ipv4Addr> for Netv4Addr {
	fn from(addr: Ipv4Addr) -> Self {
		Self::new(addr, Ipv4Addr::from(u32::max_value()))
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn uses_max_netmask() {
		let addr: Ipv4Addr = "192.0.2.42".parse().unwrap();
		let netaddr: Netv4Addr = Netv4Addr::from(addr);
		assert_eq!(
			netaddr,
			Netv4Addr::new(addr, Ipv4Addr::from(u32::max_value()))
		);
	}
}
