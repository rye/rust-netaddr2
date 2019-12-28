use netaddr2::NetAddr;

use std::net::IpAddr;

fn main() {
	// Here we parse the netaddr 127.0.24.0/8 into a NetAddr struct.
	//
	// NB: 127.0.24.0/8 represents network identified by the network
	// address 127.0.0.0 and the netmask 255.0.0.0.
	//
	// (When one parses as a NetAddr, the irrelevant network bits are
	// removed by masking; the mask is applied to the supplied address
	// so that only the relevant bits remain as they were.)
	let net: NetAddr = "127.0.24.0/8".parse().unwrap();

	// Sanity checks.  This demonstrates what has happened by forming a
	// NetAddr.
	assert_eq!(net.addr(), "127.0.0.0".parse::<IpAddr>().unwrap());
	assert_eq!(net.mask(), "255.0.0.0".parse::<IpAddr>().unwrap());

	// Now, to check containment, just this:
	use netaddr2::Contains;
	assert!(net.contains(&"127.42.33.87".parse::<IpAddr>().unwrap()));
	assert!(net.contains(&"127.0.0.1".parse::<IpAddr>().unwrap()));
	assert!(!net.contains(&"128.0.0.0".parse::<IpAddr>().unwrap()));
}
