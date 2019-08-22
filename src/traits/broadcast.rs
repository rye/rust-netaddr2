pub trait Broadcast {
	type Output;

	fn broadcast(&self) -> Self::Output;
}

#[cfg(test)]
mod tests {
	use super::Broadcast;

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
