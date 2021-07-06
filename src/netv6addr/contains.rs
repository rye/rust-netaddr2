use super::Netv6Addr;
use crate::traits::Contains;
use crate::traits::Mask;

impl Contains<std::net::IpAddr> for Netv6Addr {
	fn contains(&self, other: &std::net::IpAddr) -> bool {
		match other {
			std::net::IpAddr::V6(other) => self.contains(other),
			_ => false,
		}
	}
}

impl Contains<std::net::Ipv6Addr> for Netv6Addr {
	fn contains(&self, other: &std::net::Ipv6Addr) -> bool {
		other.mask(&self.mask()) == self.addr()
	}
}

impl Contains<crate::NetAddr> for Netv6Addr {
	fn contains(&self, other: &crate::NetAddr) -> bool {
		match other {
			crate::NetAddr::V6(other) => self.contains(other),
			_ => false,
		}
	}
}

impl Contains<Netv6Addr> for Netv6Addr {
	fn contains(&self, other: &Netv6Addr) -> bool {
		other.addr().mask(&self.mask()) == self.addr()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::net::Ipv6Addr;

	#[test]
	fn ip() {
		let net: Netv6Addr = "2001:db8:d00b::/48".parse().unwrap();
		assert!(net.contains(&Ipv6Addr::new(0x2001, 0x0db8, 0xd00b, 0, 0, 0, 0, 0x0001)));
		assert!(net.contains(&Ipv6Addr::new(
			0x2001, 0x0db8, 0xd00b, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff
		)));
		assert!(!net.contains(&Ipv6Addr::new(0x2001, 0x0db8, 0xd00c, 0, 0, 0, 0, 1)));
	}

	#[test]
	fn net() {
		let net: Netv6Addr = "2001:db8:d000::/40".parse().unwrap();
		let net_inner: Netv6Addr = "2001:db8:d00b::/48".parse().unwrap();
		assert!(net.contains(&net_inner));
	}
}
