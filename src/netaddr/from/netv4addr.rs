use crate::{NetAddr, Netv4Addr};

impl From<Netv4Addr> for NetAddr {
	fn from(netaddr: Netv4Addr) -> Self {
		Self::V4(netaddr)
	}
}
