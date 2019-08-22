use crate::{NetAddr, Netv6Addr};

impl From<Netv6Addr> for NetAddr {
	fn from(netaddr: Netv6Addr) -> Self {
		Self::V6(netaddr)
	}
}
