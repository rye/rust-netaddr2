use super::*;

#[test]
fn v4_seems_correct() {
	let net: NetAddr = "127.0.0.1/8".parse().unwrap();
	assert!(net.contains(&IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))));
	assert!(net.contains(&IpAddr::V4(Ipv4Addr::new(127, 127, 255, 1))));
	assert!(!net.contains(&IpAddr::V4(Ipv4Addr::new(64, 0, 0, 0))));
}

#[test]
fn v6_seems_correct() {
	let net: NetAddr = "127.0.0.1/8".parse().unwrap();
	assert!(net.contains(&IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))));
	assert!(net.contains(&IpAddr::V4(Ipv4Addr::new(127, 127, 255, 1))));
	assert!(!net.contains(&IpAddr::V4(Ipv4Addr::new(64, 0, 0, 0))));
}
