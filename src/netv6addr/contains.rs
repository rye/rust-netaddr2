use super::Netv6Addr;
use crate::traits::Contains;
use crate::traits::Mask;

impl Contains for Netv6Addr {
	fn contains<T: Copy>(&self, other: &T) -> bool
	where
		Self: From<T>,
	{
		let other: Self = Self::from(*other);
		other.addr().mask(&self.mask()) == *self.addr()
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
