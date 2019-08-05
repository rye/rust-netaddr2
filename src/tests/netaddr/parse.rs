use super::*;

#[test]
fn invalid_is_safe() {
	let _: Result<NetAddr, _> = "zoop".parse::<NetAddr>();
}

#[test]
fn v4_netv4addr_cidr_correct_network() {
	let input: (String, Ipv4Addr, Ipv4Addr) = rnd_v4_cidr();
	let net: Result<Netv4Addr, _> = input.0.parse();

	assert!(net.is_ok());

	let net: Netv4Addr = net.unwrap();

	assert_eq!(net.addr(), &input.1);
	assert_eq!(net.mask(), &input.2);
}

#[test]
fn v6_netv6addr_cidr_correct_network() {
	let input: (String, Ipv6Addr, Ipv6Addr) = rnd_v6_cidr();
	let net: Result<Netv6Addr, _> = input.0.parse();

	assert!(net.is_ok());

	let net: Netv6Addr = net.unwrap();

	assert_eq!(net.addr(), &input.1);
	assert_eq!(net.mask(), &input.2);
}

#[test]
fn v4_correct_network() {
	let net: NetAddr = "192.0.2.0/32".parse().unwrap();
	assert_eq!(net.mask(), IpAddr::V4(Ipv4Addr::new(255, 255, 255, 255)));
	assert_eq!(net.addr(), IpAddr::V4(Ipv4Addr::new(192, 0, 2, 0)));
}

#[test]
fn v4_localhost_8() {
	let net: NetAddr = "127.0.0.1/8".parse().unwrap();
	assert_eq!(net.mask(), IpAddr::V4(Ipv4Addr::new(255, 0, 0, 0)));
	assert_eq!(net.addr(), IpAddr::V4(Ipv4Addr::new(127, 0, 0, 0)));
}

#[test]
fn v4_cidr_22() {
	let net: NetAddr = "192.168.16.1/22".parse().unwrap();
	assert_eq!(net.mask(), IpAddr::V4(Ipv4Addr::new(255, 255, 252, 0)));
	assert_eq!(net.addr(), IpAddr::V4(Ipv4Addr::new(192, 168, 16, 0)));
}

#[test]
fn v4_extended_localhost() {
	let net: NetAddr = "127.0.0.1 255.0.0.0".parse().unwrap();
	assert_eq!(net.mask(), IpAddr::V4(Ipv4Addr::new(255, 0, 0, 0)));
	assert_eq!(net.addr(), IpAddr::V4(Ipv4Addr::new(127, 0, 0, 0)));
}

#[test]
fn v4_slashed_localhost() {
	let net: NetAddr = "127.0.0.1/255.0.0.0".parse().unwrap();
	assert_eq!(net.mask(), IpAddr::V4(Ipv4Addr::new(255, 0, 0, 0)));
	assert_eq!(net.addr(), IpAddr::V4(Ipv4Addr::new(127, 0, 0, 0)));
}

#[test]
fn v6_cidr_8() {
	let net: NetAddr = "ff02::1/8".parse().unwrap();
	assert_eq!(
		net.mask(),
		IpAddr::V6(Ipv6Addr::new(0xff00, 0, 0, 0, 0, 0, 0, 0x0000))
	);
	assert_eq!(
		net.addr(),
		IpAddr::V6(Ipv6Addr::new(0xff00, 0, 0, 0, 0, 0, 0, 0x0000))
	);
}

#[test]
fn v6_cidr_128() {
	let net: NetAddr = "ff02::1/128".parse().unwrap();
	assert_eq!(
		net.mask(),
		IpAddr::V6(Ipv6Addr::new(
			0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff
		))
	);
	assert_eq!(
		net.addr(),
		IpAddr::V6(Ipv6Addr::new(0xff02, 0, 0, 0, 0, 0, 0, 0x0001))
	);
}

#[test]
fn v6_extended() {
	let net: NetAddr = "ff02::1 ffff::0".parse().unwrap();
	assert_eq!(
		net.mask(),
		IpAddr::V6(Ipv6Addr::new(0xffff, 0, 0, 0, 0, 0, 0, 0))
	);
	assert_eq!(
		net.addr(),
		IpAddr::V6(Ipv6Addr::new(0xff02, 0, 0, 0, 0, 0, 0, 0))
	);
}

#[test]
fn v6_slashed() {
	let net: NetAddr = "ff02::1/128".parse().unwrap();
	assert_eq!(
		net.mask(),
		IpAddr::V6(Ipv6Addr::new(
			0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff
		))
	);
	assert_eq!(
		net.addr(),
		IpAddr::V6(Ipv6Addr::new(0xff02, 0, 0, 0, 0, 0, 0, 0x0001))
	);
}

#[test]
fn addr_only() {
	let net: NetAddr = "127.0.0.1/zoop".parse().unwrap();
	assert_eq!(net, "127.0.0.1/32".parse().unwrap());
}

#[test]
fn addr_no_mask_returns_full_bitstring() {
	let net: NetAddr = "127.0.0.1/zoop".parse().unwrap();
	assert_eq!(net, "127.0.0.1/32".parse().unwrap());
	let net: NetAddr = "ff02::1/zoop".parse().unwrap();
	assert_eq!(net, "ff02::1/128".parse().unwrap());
}

#[test]
fn non_addr_passes_out_error() {
	let result = "zoop".parse::<NetAddr>();
	assert_eq!(
		result,
		Err(NetAddrError::ParseError(
			"could not split provided input".to_string()
		))
	);
}
