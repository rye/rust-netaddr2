use super::*;

#[test]
fn invalid_is_safe() {
	let _: Result<NetAddr, _> = "zoop".parse::<NetAddr>();
}

#[test]
fn v4_correct_network() {
	let net: NetAddr = "192.0.2.0/32".parse().unwrap();
	assert_eq!(net.netmask, IpAddr::V4(Ipv4Addr::new(255, 255, 255, 255)));
	assert_eq!(net.network, IpAddr::V4(Ipv4Addr::new(192, 0, 2, 0)));
}

#[test]
fn v4_localhost() {
	let net: NetAddr = "127.0.0.1/8".parse().unwrap();
	assert_eq!(net.netmask, IpAddr::V4(Ipv4Addr::new(255, 0, 0, 0)));
	assert_eq!(net.network, IpAddr::V4(Ipv4Addr::new(127, 0, 0, 0)));
}

#[test]
fn v4_cidr_22() {
	let net: NetAddr = "192.168.16.1/22".parse().unwrap();
	assert_eq!(net.netmask, IpAddr::V4(Ipv4Addr::new(255, 255, 252, 0)));
	assert_eq!(net.network, IpAddr::V4(Ipv4Addr::new(192, 168, 16, 0)));
}

#[test]
fn v4_extended_localhost() {
	let net: NetAddr = "127.0.0.1 255.0.0.0".parse().unwrap();
	assert_eq!(net.netmask, IpAddr::V4(Ipv4Addr::new(255, 0, 0, 0)));
	assert_eq!(net.network, IpAddr::V4(Ipv4Addr::new(127, 0, 0, 0)));
}

#[test]
fn v4_slashed_localhost() {
	let net: NetAddr = "127.0.0.1/255.0.0.0".parse().unwrap();
	assert_eq!(net.netmask, IpAddr::V4(Ipv4Addr::new(255, 0, 0, 0)));
	assert_eq!(net.network, IpAddr::V4(Ipv4Addr::new(127, 0, 0, 0)));
}

#[test]
fn v6_cidr_8() {
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

#[test]
fn v6_cidr_128() {
	let net: NetAddr = "ff02::1/128".parse().unwrap();
	assert_eq!(
		net.netmask,
		IpAddr::V6(Ipv6Addr::new(
			0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff
		))
	);
	assert_eq!(
		net.network,
		IpAddr::V6(Ipv6Addr::new(0xff02, 0, 0, 0, 0, 0, 0, 0x0001))
	);
}
