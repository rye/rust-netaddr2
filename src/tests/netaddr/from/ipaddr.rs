use super::*;

#[test]
fn v4_returns_full_netmask() {
	let addr: IpAddr = "192.0.2.42".parse().unwrap();
	assert_eq!(
		NetAddr::from(addr),
		NetAddr {
			network: addr,
			netmask: IpAddr::V4(Ipv4Addr::from(0xff_ff_ff_ff))
		}
	);
}

#[test]
fn v6_returns_full_netmask() {
	let addr: IpAddr = "2001:db8:dead:beef::42".parse().unwrap();
	assert_eq!(
		NetAddr::from(addr),
		NetAddr {
			network: addr,
			netmask: IpAddr::V6(Ipv6Addr::from(0xffff_ffff_ffff_ffff_ffff_ffff_ffff_ffff))
		}
	);
}
