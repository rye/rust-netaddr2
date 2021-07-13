use std::net::{IpAddr, Ipv4Addr};

use crate::{netaddr::NetAddr, netv4addr::Netv4Addr, traits::Contains, traits::Mask};

impl Contains<IpAddr> for Netv4Addr {
	fn contains(&self, other: &IpAddr) -> bool {
		match other {
			IpAddr::V4(other) => self.contains(other),
			IpAddr::V6(_) => false,
		}
	}
}

impl Contains<Ipv4Addr> for Netv4Addr {
	fn contains(&self, other: &Ipv4Addr) -> bool {
		other.mask(&self.mask()) == self.addr()
	}
}

impl Contains<NetAddr> for Netv4Addr {
	fn contains(&self, other: &NetAddr) -> bool {
		match other {
			NetAddr::V4(other) => self.contains(other),
			NetAddr::V6(_) => false,
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
		mod ipaddr {
			mod v4 {
				use std::net::IpAddr;

				use crate::{netv4addr::Netv4Addr, traits::Contains};

				#[test]
				fn loopback_contains_some_loopback_ips() {
					let net: Netv4Addr = "127.0.0.1/8".parse().unwrap();
					assert!(net.contains(&"127.0.0.1".parse::<IpAddr>().unwrap()));
					assert!(net.contains(&"127.128.42.0".parse::<IpAddr>().unwrap()));
				}

				#[test]
				fn loopback_does_not_contain_some_others() {
					let net: Netv4Addr = "127.0.0.1/8".parse().unwrap();
					assert!(!net.contains(&"64.0.0.0".parse::<IpAddr>().unwrap()));
				}
			}

			mod v6 {
				use std::net::{IpAddr, Ipv6Addr};

				use crate::{netv4addr::Netv4Addr, traits::Contains};

				#[test]
				fn returns_false() {
					let net: Netv4Addr = "127.0.0.1/8".parse().unwrap();
					assert!(!net.contains(&IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0))));
				}
			}
		}

		mod ipv4addr {
			use std::net::Ipv4Addr;

			use crate::{netv4addr::Netv4Addr, traits::Contains};

			#[test]
			fn loopback_contains_loopback() {
				let net: Netv4Addr = "127.0.0.1/8".parse().unwrap();
				assert!(net.contains(&Ipv4Addr::new(127, 0, 0, 1)));
				assert!(net.contains(&Ipv4Addr::new(127, 127, 255, 1)));
				assert!(!net.contains(&Ipv4Addr::new(64, 0, 0, 0)));
			}
		}

		mod netaddr {

			mod v4 {
				use crate::{netaddr::NetAddr, netv4addr::Netv4Addr, traits::Contains};

				#[test]
				fn loopback_contains_subnet() {
					let net: Netv4Addr = "127.0.0.1/8".parse().unwrap();
					let net_subnet: NetAddr = "127.127.42.1/24".parse().unwrap();
					assert!(net.contains(&net_subnet));
				}
			}

			mod v6 {
				use crate::{netaddr::NetAddr, netv4addr::Netv4Addr, traits::Contains};

				#[test]
				fn loopback_does_not_contain_some_v6() {
					let net: Netv4Addr = "127.0.0.1/8".parse().unwrap();
					let other: NetAddr = "::1/17".parse().unwrap();
					assert!(!net.contains(&other));
				}
			}
		}

		mod netv4addr {
			use crate::{netv4addr::Netv4Addr, traits::Contains};

			#[test]
			fn loopback_contains_subnet() {
				let net: Netv4Addr = "127.0.0.1/8".parse().unwrap();
				let net_subnet: Netv4Addr = "127.127.42.1/24".parse().unwrap();
				assert!(net.contains(&net_subnet));
			}
		}
	}

	mod non_cidr {
		use std::net::Ipv4Addr;

		use crate::{Contains, Netv4Addr};

		#[test]
		fn ip() {
			let net: Netv4Addr = "127.255.255.0/255.127.127.0".parse().unwrap();

			assert!(net.contains(&Ipv4Addr::new(127, 255, 127, 0)));
			assert!(net.contains(&Ipv4Addr::new(127, 255, 127, 255)));
			assert!(!net.contains(&Ipv4Addr::new(255, 127, 127, 0)));
		}
	}
}
