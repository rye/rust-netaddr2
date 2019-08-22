use super::Netv4Addr;
use crate::Broadcast;
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
