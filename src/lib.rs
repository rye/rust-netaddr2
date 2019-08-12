use std::net::{Ipv4Addr, Ipv6Addr};

impl From<Ipv4Addr> for NetAddr {
	fn from(addr: Ipv4Addr) -> Self {
		NetAddr::V4(Netv4Addr::from(addr))
	}
}

impl From<Ipv4Addr> for Netv4Addr {
	fn from(addr: Ipv4Addr) -> Self {
		Self::new(addr, Ipv4Addr::from(u32::max_value()))
	}
}

impl From<Ipv6Addr> for NetAddr {
	fn from(addr: Ipv6Addr) -> Self {
		NetAddr::V6(Netv6Addr::from(addr))
	}
}

impl From<Ipv6Addr> for Netv6Addr {
	fn from(addr: Ipv6Addr) -> Self {
		Self::new(addr, Ipv6Addr::from(u128::max_value()))
	}
}

mod broadcast;
mod contains;
mod mask;
mod merge;
pub use broadcast::*;
pub use contains::*;
pub use mask::*;
pub use merge::*;

mod netaddr;
mod netv4addr;
mod netv6addr;
pub use netaddr::*;
pub use netv4addr::*;
pub use netv6addr::*;

mod netaddr_error;
pub use netaddr_error::*;

#[cfg(test)]
mod tests;
