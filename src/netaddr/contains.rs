use super::NetAddr;
use crate::traits::Contains;

impl Contains<std::net::IpAddr> for NetAddr {
	fn contains(&self, other: &std::net::IpAddr) -> bool {
		match self {
			Self::V4(netaddr) => netaddr.contains(other),
			Self::V6(netaddr) => netaddr.contains(other),
		}
	}
}

impl Contains<std::net::Ipv4Addr> for NetAddr {
	fn contains(&self, other: &std::net::Ipv4Addr) -> bool {
		match self {
			Self::V4(netaddr) => netaddr.contains(other),
			_ => false,
		}
	}
}

impl Contains<std::net::Ipv6Addr> for NetAddr {
	fn contains(&self, other: &std::net::Ipv6Addr) -> bool {
		match self {
			Self::V6(netaddr) => netaddr.contains(other),
			_ => false,
		}
	}
}

impl Contains<NetAddr> for NetAddr {
	fn contains(&self, other: &NetAddr) -> bool {
		match self {
			Self::V4(netaddr) => netaddr.contains(other),
			Self::V6(netaddr) => netaddr.contains(other),
		}
	}
}

impl Contains<crate::Netv4Addr> for NetAddr {
	fn contains(&self, other: &crate::Netv4Addr) -> bool {
		match self {
			Self::V4(netaddr) => netaddr.contains(other),
			_ => false,
		}
	}
}

impl Contains<crate::Netv6Addr> for NetAddr {
	fn contains(&self, other: &crate::Netv6Addr) -> bool {
		match self {
			Self::V6(netaddr) => netaddr.contains(other),
			_ => false,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::NetAddr;
	use std::net::IpAddr;

	macro_rules! assert_contains {
		($a:expr, $b:expr) => {
			assert!($a.contains(&$b));
		};
		($a:expr, $b:literal # $b_ty:ty) => {
			assert!($a.contains(&pu!($b # $b_ty)));
		};
		($a:literal # $a_ty:ty, $b:expr) => {
			assert!(pu!($a # $a_ty).contains(&$b));
		};
		($a:literal # $a_ty:ty, $b:literal # $b_ty:ty) => {
			assert!(pu!($a # $a_ty).contains(&pu!($b # $b_ty)));
		};
	}

	macro_rules! assert_not_contains {
		($a:expr, $b:expr) => {
			assert!(!$a.contains(&$b));
		};
		($a:expr, $b:literal # $b_ty:ty) => {
			assert!(!$a.contains(&pu!($b # $b_ty)));
		};
		($a:literal # $a_ty:ty, $b:expr) => {
			assert!(!pu!($a # $a_ty).contains(&$b));
		};
		($a:literal # $a_ty:ty, $b:literal # $b_ty:ty) => {
			assert!(!pu!($a # $a_ty).contains(&pu!($b # $b_ty)));
		};
	}

	mod v4 {
		use super::*;
		use crate::{Netv4Addr, Netv6Addr};
		use std::net::{Ipv4Addr, Ipv6Addr};

		#[test]
		fn ipaddr_v4() {
			let net = pu!("127.0.0.1/8" # NetAddr);
			assert_contains!(net, "127.0.0.1" # IpAddr);
			assert_contains!(net, "127.127.255.1" # IpAddr);
			assert_not_contains!(net, "64.73.69.2" # IpAddr);
		}

		#[test]
		fn ipaddr_v6() {
			let net = pu!("127.0.0.1/8" # NetAddr);
			assert_not_contains!(net, "ff02::1" # IpAddr);
			assert_not_contains!(net, "::ffff:127.0.0.1" # IpAddr);
		}

		#[test]
		fn ipv4addr() {
			let net = pu!("127.0.0.1/8" # NetAddr);
			assert_contains!(net, "127.0.0.1" # Ipv4Addr);
			assert_contains!(net, "127.127.255.1" # Ipv4Addr);
			assert_not_contains!(net, "64.73.69.2" # Ipv4Addr);
		}

		#[test]
		fn ipv6addr() {
			let net = pu!("127.0.0.1/8" # NetAddr);
			assert_not_contains!(net, "ff02::1" # Ipv6Addr);
			assert_not_contains!(net, "::ffff:127.0.0.1" # Ipv6Addr);
		}

		#[test]
		fn netaddr_v4() {
			let net = pu!("127.0.0.1/8" # NetAddr);
			assert_contains!(net, "127.128.0.1/24" # NetAddr);
		}

		#[test]
		fn netaddr_v6() {
			let net = pu!("127.0.0.1/8" # NetAddr);
			assert_not_contains!(net, "ff02::1/16" # NetAddr);
			assert_not_contains!(net, "::ffff:127.0.0.1/96" # NetAddr);
		}

		#[test]
		fn netv4addr() {
			let net: NetAddr = "127.0.0.1/8".parse().unwrap();
			assert_contains!(net, "127.0.0.1/24" # Netv4Addr);
			assert_contains!(net, "127.127.255.63/24" # Netv4Addr);
			assert_not_contains!(net, "64.73.81.69/24" # Netv4Addr);
		}

		#[test]
		fn netv6addr() {
			let net: NetAddr = "127.0.0.1/8".parse().unwrap();
			assert_not_contains!(net, "ff02::1/16" # Netv6Addr);
			assert_not_contains!(net, "::ffff:127.0.0.1/96" # Netv6Addr);
		}
	}

	mod v6 {
		use super::*;
		use crate::{Netv4Addr, Netv6Addr};
		use std::net::{Ipv4Addr, Ipv6Addr};

		#[test]
		fn ipaddr_v4() {
			let net = pu!("2001:db8:dead:beef::/64" # NetAddr);
			assert_not_contains!(net, "127.0.0.1" # IpAddr);
			assert_not_contains!(net, "127.127.255.1" # IpAddr);
			assert_not_contains!(net, "64.73.69.2" # IpAddr);
		}

		#[test]
		fn ipaddr_v6() {
			let net = pu!("2001:db8:dead:beef::/64" # NetAddr);
			assert_not_contains!(net, "ff02::1" # IpAddr);
			assert_not_contains!(net, "::ffff:127.0.0.1" # IpAddr);
			assert_contains!(net, "2001:db8:dead:beef::1" # IpAddr);
			assert_contains!(net, "2001:db8:dead:beef:c0f:fee:dab:69" # IpAddr);
		}

		#[test]
		fn ipv4addr() {
			let net = pu!("2001:db8:dead:beef::/64" # NetAddr);
			assert_not_contains!(net, "127.0.0.1" # Ipv4Addr);
			assert_not_contains!(net, "127.127.255.1" # Ipv4Addr);
			assert_not_contains!(net, "64.73.69.2" # Ipv4Addr);
		}

		#[test]
		fn ipv6addr() {
			let net = pu!("2001:db8:dead:beef::/64" # NetAddr);
			assert_not_contains!(net, "ff02::1" # Ipv6Addr);
			assert_not_contains!(net, "::ffff:127.0.0.1" # Ipv6Addr);
			assert_contains!(net, "2001:db8:dead:beef::1" # Ipv6Addr);
			assert_contains!(net, "2001:db8:dead:beef:c0f:fee:dab:69" # Ipv6Addr);
		}

		#[test]
		fn netaddr_v4() {
			let net = pu!("2001:db8:dead:beef::/64" # NetAddr);
			assert_not_contains!(net, "127.128.0.1/24" # NetAddr);
			assert_contains!(net, "2001:db8:dead:beef::1/64" # NetAddr);
			assert_contains!(net, "2001:db8:dead:beef:c0f:fee::/96" # NetAddr);
		}

		#[test]
		fn netaddr_v6() {
			let net = pu!("2001:db8:dead:beef::/64" # NetAddr);
			assert_not_contains!(net, "ff02::1/16" # NetAddr);
			assert_not_contains!(net, "::ffff:127.0.0.1/96" # NetAddr);
		}

		#[test]
		fn netv4addr() {
			let net: NetAddr = "2001:db8:dead:beef::/64".parse().unwrap();
			assert_not_contains!(net, "127.0.0.1/24" # Netv4Addr);
			assert_not_contains!(net, "127.127.255.63/24" # Netv4Addr);
			assert_not_contains!(net, "64.73.81.69/24" # Netv4Addr);
		}

		#[test]
		fn netv6addr() {
			let net: NetAddr = "2001:db8:dead:beef::/64".parse().unwrap();
			assert_not_contains!(net, "ff02::1/16" # Netv6Addr);
			assert_not_contains!(net, "::ffff:127.0.0.1/96" # Netv6Addr);
			assert_contains!(net, "2001:db8:dead:beef::1/64" # Netv6Addr);
			assert_contains!(net, "2001:db8:dead:beef:c0f:fee::/96" # Netv6Addr);
		}
	}
}
