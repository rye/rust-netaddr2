use std::net::IpAddr;

extern crate netaddr2;
use netaddr2::NetAddr;

fn main() {
	let net: NetAddr = "192.0.2.0/24".parse().unwrap();
	assert_eq!(net.network, IpAddr::V4("192.0.2.0".parse().unwrap()));
	assert_eq!(net.netmask, IpAddr::V4("255.255.255.0".parse().unwrap()));

	let net: NetAddr = "2001:db8:dead:beef::/32".parse().unwrap();
	assert_eq!(net.network, IpAddr::V6("2001:db8::".parse().unwrap()));
	assert_eq!(net.netmask, IpAddr::V6("ffff:ffff::".parse().unwrap()));
}
