use super::Netv4Addr;
use crate::traits::Contains;
use crate::traits::Mask;

impl Contains for Netv4Addr {
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
	use std::net::Ipv4Addr;

	#[test]
	fn ip() {
		let net: Netv4Addr = "127.0.0.1/8".parse().unwrap();
		assert!(net.contains(&Ipv4Addr::new(127, 0, 0, 1)));
		assert!(net.contains(&Ipv4Addr::new(127, 127, 255, 1)));
		assert!(!net.contains(&Ipv4Addr::new(64, 0, 0, 0)));
	}

	#[test]
	fn net() {
		let net: Netv4Addr = "127.0.0.1/8".parse().unwrap();
		let net_inner: Netv4Addr = "127.128.0.1/24".parse().unwrap();
		assert!(net.contains(&net_inner));
	}
}
