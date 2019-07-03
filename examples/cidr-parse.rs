use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

extern crate netaddr2;
use netaddr2::NetAddr;

fn main() {
	let net: NetAddr = "127.0.0.1/8".parse().unwrap();
	assert_eq!(net.netmask, IpAddr::V4(Ipv4Addr::new(255, 0, 0, 0)));
	assert_eq!(net.network, IpAddr::V4(Ipv4Addr::new(127, 0, 0, 0)));

	let net: NetAddr = "192.168.16.1/22".parse().unwrap();
	assert_eq!(net.netmask, IpAddr::V4(Ipv4Addr::new(255, 255, 252, 0)));
	assert_eq!(net.network, IpAddr::V4(Ipv4Addr::new(192, 168, 16, 0)));

	let net: NetAddr = "ff02::1/8".parse().unwrap();
	assert_eq!(
		net.netmask,
		IpAddr::V6(Ipv6Addr::new(0xff00, 0, 0, 0, 0, 0, 0, 0x0000))
	);
	assert_eq!(
		net.network,
		IpAddr::V6(Ipv6Addr::new(0xff00, 0, 0, 0, 0, 0, 0, 0x0000))
	);
}
