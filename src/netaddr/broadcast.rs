use super::NetAddr;
use crate::Broadcast;
use std::net::IpAddr;

impl Broadcast for NetAddr {
	type Output = Option<IpAddr>;

	fn broadcast(&self) -> Self::Output {
		match self {
			Self::V4(netaddr) => Some(IpAddr::from(netaddr.broadcast())),
			_ => None,
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
		fn returns_correct_address() {
			let net: NetAddr = "127.0.0.1/8".parse().unwrap();
			assert_eq!(
				net.broadcast().unwrap(),
				IpAddr::V4(Ipv4Addr::new(127, 255, 255, 255))
			);

			let net: NetAddr = "192.168.69.25/29".parse().unwrap();
			assert_eq!(
				net.broadcast().unwrap(),
				IpAddr::V4(Ipv4Addr::new(192, 168, 69, 31))
			);

			let net: NetAddr = "192.168.128.127/32".parse().unwrap();
			assert_eq!(
				net.broadcast().unwrap(),
				IpAddr::V4(Ipv4Addr::new(192, 168, 128, 127))
			);
		}
	}

	mod v6 {
		use super::*;

		#[test]
		fn returns_none() {
			let net: NetAddr = "fe80::1/64".parse().unwrap();
			assert_eq!(net.broadcast(), None);
		}
	}
}
