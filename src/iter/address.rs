use crate::NetAddr;
use crate::Netv4Addr;
use crate::Netv6Addr;

use crate::Contains;

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

pub struct NetAddrAddressIterator {
	net: NetAddr,
	cur: Option<IpAddr>,
}

impl Iterator for NetAddrAddressIterator {
	type Item = IpAddr;

	fn next(&mut self) -> Option<Self::Item> {
		let cur: Option<IpAddr> = self.cur;
		let next: Option<IpAddr> = match cur {
			Some(IpAddr::V4(ipv4)) => u32::from(ipv4)
				.checked_add(1)
				.map(|next: u32| IpAddr::V4(Ipv4Addr::from(next))),
			Some(IpAddr::V6(ipv6)) => u128::from(ipv6)
				.checked_add(1)
				.map(|next: u128| IpAddr::V6(Ipv6Addr::from(next))),
			None => None,
		};

		match (cur, next) {
			(Some(cur), Some(next)) => {
				if self.net.contains(&cur) {
					self.cur = Some(next);
					Some(cur)
				} else {
					None
				}
			}
			(Some(cur), None) => {
				self.cur = next;
				Some(cur)
			}
			(None, _) => None,
		}
	}
}

pub struct Netv4AddrAddressIterator {
	net: Netv4Addr,
	cur: Ipv4Addr,
}

pub struct Netv6AddrAddressIterator {
	net: Netv6Addr,
	cur: Ipv6Addr,
}

#[cfg(test)]
mod tests {
}
