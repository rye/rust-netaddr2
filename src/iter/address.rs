use crate::NetAddr;
use crate::Netv4Addr;
use crate::Netv6Addr;

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

pub struct NetAddrAddressIterator {
	net: NetAddr,
	cur: Option<IpAddr>,
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
