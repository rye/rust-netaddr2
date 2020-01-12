use crate::{NetAddr, Netv6Addr};

impl From<Netv6Addr> for NetAddr {
	fn from(netaddr: Netv6Addr) -> Self {
		Self::V6(netaddr)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn wraps_given_value_in_v6_variant() {
		let addr: Netv6Addr = "2001:db8:dead:beef::42/40".parse().unwrap();
		let netaddr: NetAddr = NetAddr::from(addr);
		assert_eq!(netaddr, NetAddr::V6(addr));
	}
}
