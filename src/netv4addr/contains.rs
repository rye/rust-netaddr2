use super::Netv4Addr;
use crate::traits::Contains;
use crate::traits::Mask;

impl Contains<std::net::IpAddr> for Netv4Addr {
	fn contains(&self, other: &std::net::IpAddr) -> bool {
		match other {
			std::net::IpAddr::V4(other) => self.contains(other),
			_ => false,
		}
	}
}

impl Contains<std::net::Ipv4Addr> for Netv4Addr {
	fn contains(&self, other: &std::net::Ipv4Addr) -> bool {
		other.mask(&self.mask()) == self.addr()
	}
}

impl Contains<crate::NetAddr> for Netv4Addr {
	fn contains(&self, other: &crate::NetAddr) -> bool {
		match other {
			crate::NetAddr::V4(other) => self.contains(other),
			_ => false,
		}
	}
}

impl Contains<Netv4Addr> for Netv4Addr {
	fn contains(&self, other: &Netv4Addr) -> bool {
		other.addr().mask(&self.mask()) == self.addr()
	}
}

#[cfg(test)]
mod tests {
	mod cidr {
		use crate::{Contains, Netv4Addr};
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

	mod non_cidr {
		use crate::{Contains, Netv4Addr};
		use std::net::Ipv4Addr;

		#[test]
		fn ip() {
			let net: Netv4Addr = "127.255.255.0/255.127.127.0".parse().unwrap();

			assert!(net.contains(&Ipv4Addr::new(127, 255, 127, 0)));
			assert!(net.contains(&Ipv4Addr::new(127, 255, 127, 255)));
			assert!(!net.contains(&Ipv4Addr::new(255, 127, 127, 0)));
		}
	}
}
