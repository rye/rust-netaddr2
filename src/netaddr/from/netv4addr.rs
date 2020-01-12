use crate::{NetAddr, Netv4Addr};

impl From<Netv4Addr> for NetAddr {
	fn from(netaddr: Netv4Addr) -> Self {
		Self::V4(netaddr)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn wraps_given_value_in_v4_variant() {
		let addr: Netv4Addr = "192.0.2.42/27".parse().unwrap();
		let netaddr: NetAddr = NetAddr::from(addr);
		assert_eq!(netaddr, NetAddr::V4(addr));
	}
}
