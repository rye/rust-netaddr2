use crate::netv6addr::Netv6Addr;
use core::cmp::Ordering;

impl PartialOrd for Netv6Addr {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		match self.addr().partial_cmp(other.addr()) {
			Some(Ordering::Equal) => self.mask().partial_cmp(other.mask()),
			Some(ordering) => Some(ordering),
			None => None,
		}
	}
}
