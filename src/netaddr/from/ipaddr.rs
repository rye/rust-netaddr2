use crate::NetAddr;
use std::net::IpAddr;

impl From<IpAddr> for NetAddr {
	fn from(addr: IpAddr) -> Self {
		match addr {
			IpAddr::V4(addr) => Self::from(addr),
			IpAddr::V6(addr) => Self::from(addr),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	mod v4 {
		use super::*;
		use crate::Netv4Addr;
		use std::net::Ipv4Addr;

		#[test]
		fn uses_max_netmask() {
			let addr: IpAddr = "192.0.2.42".parse().unwrap();
			let netaddr: NetAddr = NetAddr::from(addr);

			assert_eq!(
				netaddr,
				NetAddr::V4(Netv4Addr::new(
					Ipv4Addr::new(192, 0, 2, 42),
					Ipv4Addr::from(u32::max_value())
				))
			);
		}
	}

	mod v6 {
		use super::*;
		use crate::Netv6Addr;
		use std::net::Ipv6Addr;

		#[test]
		fn uses_max_netmask() {
			let addr: IpAddr = "2001:db8:dead:beef::42".parse().unwrap();
			let netaddr: NetAddr = NetAddr::from(addr);

			assert_eq!(
				netaddr,
				NetAddr::V6(Netv6Addr::new(
					Ipv6Addr::new(0x2001, 0xdb8, 0xdead, 0xbeef, 0, 0, 0, 0x0042),
					Ipv6Addr::from(u128::max_value())
				))
			);
		}
	}
}
