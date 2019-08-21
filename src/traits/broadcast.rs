pub trait Broadcast {
	type Output;

	fn broadcast(&self) -> Self::Output;
}

#[cfg(test)]
mod tests {
	use super::Broadcast;

	mod netaddr {
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

	mod netv4addr {
		use super::*;
		use crate::Netv4Addr;
		use std::net::Ipv4Addr;

		#[test]
		fn returns_correct_address() {
			let net: Netv4Addr = "127.0.0.1/8".parse().unwrap();
			assert_eq!(net.broadcast(), Ipv4Addr::new(127, 255, 255, 255));

			let net: Netv4Addr = "192.168.69.25/29".parse().unwrap();
			assert_eq!(net.broadcast(), Ipv4Addr::new(192, 168, 69, 31));

			let net: Netv4Addr = "192.168.128.127/32".parse().unwrap();
			assert_eq!(net.broadcast(), Ipv4Addr::new(192, 168, 128, 127));
		}
	}
}
