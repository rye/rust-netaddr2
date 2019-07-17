use super::*;

#[test]
fn v4_returns_full_netmask() {
	let addr: Ipv4Addr = "192.0.2.42".parse().unwrap();
	assert_eq!(
		NetAddr::from(addr),
		NetAddr::V4 {
			network: addr,
			netmask: Ipv4Addr::from(0xff_ff_ff_ff)
		}
	);
}

#[test]
fn v6_returns_full_netmask() {
	let addr: Ipv6Addr = "2001:db8:dead:beef::42".parse().unwrap();
	assert_eq!(
		NetAddr::from(addr),
		NetAddr::V6 {
			network: addr,
			netmask: Ipv6Addr::from(0xffff_ffff_ffff_ffff_ffff_ffff_ffff_ffff)
		}
	);
}
