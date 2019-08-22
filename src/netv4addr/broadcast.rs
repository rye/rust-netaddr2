use super::Netv4Addr;
use crate::traits::Broadcast;
use std::net::Ipv4Addr;

impl Broadcast for Netv4Addr {
	type Output = Ipv4Addr;

	fn broadcast(&self) -> Ipv4Addr {
		let netmask: u32 = self.mask().clone().into();
		let network: u32 = self.addr().clone().into();
		let broadcast: u32 = network | !netmask;
		broadcast.into()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

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
