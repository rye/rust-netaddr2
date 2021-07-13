use std::net::Ipv6Addr;

use crate::netv6addr::Netv6Addr;

impl From<Ipv6Addr> for Netv6Addr {
	fn from(addr: Ipv6Addr) -> Self {
		Self::new(addr, Ipv6Addr::from(u128::MAX))
	}
}

#[cfg(test)]
mod tests {
	use std::net::Ipv6Addr;

	use crate::netv6addr::Netv6Addr;

	#[test]
	fn uses_max_netmask() {
		let addr: Ipv6Addr = "2001:db8:dead:beef::42".parse().unwrap();
		let netaddr: Netv6Addr = Netv6Addr::from(addr);

		assert_eq!(netaddr, Netv6Addr::new(addr, Ipv6Addr::from(u128::MAX)));
	}
}
