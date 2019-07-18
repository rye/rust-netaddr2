use super::*;

#[test]
fn v4_ip() {
	let net: NetAddr = "127.0.0.1/8".parse().unwrap();
	assert!(net.contains(&IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))));
	assert!(net.contains(&IpAddr::V4(Ipv4Addr::new(127, 127, 255, 1))));
	assert!(!net.contains(&IpAddr::V4(Ipv4Addr::new(64, 0, 0, 0))));
}

#[test]
fn v4_net() {
	let net: NetAddr = "127.0.0.1/8".parse().unwrap();
	let net_inner: NetAddr = "127.128.0.1/24".parse().unwrap();
	assert!(net.contains(&net_inner));
}

#[test]
fn v6_ip() {
	let net: NetAddr = "2001:db8:d00b::/48".parse().unwrap();
	assert!(net.contains(&IpAddr::V6(Ipv6Addr::new(
		0x2001, 0x0db8, 0xd00b, 0, 0, 0, 0, 0x0001
	))));
	assert!(net.contains(&IpAddr::V6(Ipv6Addr::new(
		0x2001, 0x0db8, 0xd00b, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff
	))));
	assert!(!net.contains(&IpAddr::V6(Ipv6Addr::new(
		0x2001, 0x0db8, 0xd00c, 0, 0, 0, 0, 1
	))));
}

#[test]
fn v6_net() {
	let net: NetAddr = "2001:db8:d000::/40".parse().unwrap();
	let net_inner: NetAddr = "2001:db8:d00b::/48".parse().unwrap();
	assert!(net.contains(&net_inner));
}

#[test]
#[should_panic]
fn v4_v6_ip() {
	let net: NetAddr = "127.0.0.1/8".parse().unwrap();
	let ip: IpAddr = "2001:db8:d00b::1".parse().unwrap();
	assert!(!net.contains(&ip));
}

#[test]
#[should_panic]
fn v4_v6_net() {
	let a: NetAddr = "127.0.0.1/8".parse().unwrap();
	let b: IpAddr = "2001:db8:d0::/48".parse().unwrap();
	assert!(!a.contains(&b));
}

#[test]
#[should_panic]
fn v6_v4_ip() {
	let net: NetAddr = "2001:db8:d000::/40".parse().unwrap();
	let ip: IpAddr = "127.0.0.1".parse().unwrap();
	assert!(!net.contains(&ip));
}

#[test]
#[should_panic]
fn v6_v4_net() {
	let a: NetAddr = "2001:db8:d0::/48".parse().unwrap();
	let b: IpAddr = "127.0.0.1/8".parse().unwrap();
	assert!(!a.contains(&b));
}
