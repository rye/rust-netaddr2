use super::NetAddr;
use crate::traits::Contains;

impl Contains<std::net::IpAddr> for NetAddr {
	fn contains(&self, other: &std::net::IpAddr) -> bool {
		let other: Self = Self::from(*other);
		match (self, other) {
			(Self::V4(netaddr), Self::V4(other)) => netaddr.contains(&other),
			(Self::V6(netaddr), Self::V6(other)) => netaddr.contains(&other),
			(_, _) => false,
		}
	}
}

impl Contains<NetAddr> for NetAddr {
	fn contains(&self, other: &NetAddr) -> bool {
		let other: Self = Self::from(*other);
		match (self, other) {
			(Self::V4(netaddr), Self::V4(other)) => netaddr.contains(&other),
			(Self::V6(netaddr), Self::V6(other)) => netaddr.contains(&other),
			(_, _) => false,
		}
	}
}


#[cfg(test)]
mod tests {
	use super::*;
	use crate::NetAddr;
	use std::net::IpAddr;

	mod v4 {
		use super::*;
		use std::net::Ipv4Addr;

		#[test]
		fn ip() {
			let net: NetAddr = "127.0.0.1/8".parse().unwrap();
			assert!(net.contains(&IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))));
			assert!(net.contains(&IpAddr::V4(Ipv4Addr::new(127, 127, 255, 1))));
			assert!(!net.contains(&IpAddr::V4(Ipv4Addr::new(64, 0, 0, 0))));
		}

		#[test]
		fn net() {
			let net: NetAddr = "127.0.0.1/8".parse().unwrap();
			let net_inner: NetAddr = "127.128.0.1/24".parse().unwrap();
			assert!(net.contains(&net_inner));
		}

		#[test]
		fn v6_ip() {
			let net: NetAddr = "127.0.0.1/8".parse().unwrap();
			let ip: IpAddr = "2001:db8:d00b::1".parse().unwrap();
			assert!(!net.contains(&ip));
		}

		#[test]
		fn v6_net() {
			let a: NetAddr = "127.0.0.1/8".parse().unwrap();
			let b: NetAddr = "2001:db8:d0::/48".parse().unwrap();
			assert!(!a.contains(&b));
		}
	}

	mod v6 {
		use super::*;
		use std::net::Ipv6Addr;

		#[test]
		fn ip() {
			let net: NetAddr = "2001:db8:d00b::/48".parse().unwrap();
			assert!(net.contains(&IpAddr::V6(Ipv6Addr::new(
				0x2001, 0x0db8, 0xd00b, 0, 0, 0, 0, 0x0001
			))));
			assert!(net.contains(&IpAddr::V6(Ipv6Addr::new(
				0x2001, 0x0db8, 0xd00b, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff
			))));
			assert!(!net.contains(&IpAddr::V6(Ipv6Addr::new(
				0x2001, 0x0db8, 0xd00c, 0, 0, 0, 0, 1
			))));
		}

		#[test]
		fn net() {
			let net: NetAddr = "2001:db8:d000::/40".parse().unwrap();
			let net_inner: NetAddr = "2001:db8:d00b::/48".parse().unwrap();
			assert!(net.contains(&net_inner));
		}

		#[test]
		fn v4_ip() {
			let net: NetAddr = "2001:db8:d000::/40".parse().unwrap();
			let ip: IpAddr = "127.0.0.1".parse().unwrap();
			assert!(!net.contains(&ip));
		}

		#[test]
		fn v4_net() {
			let a: NetAddr = "2001:db8:d0::/48".parse().unwrap();
			let b: NetAddr = "127.0.0.1/8".parse().unwrap();
			assert!(!a.contains(&b));
		}
	}
}
