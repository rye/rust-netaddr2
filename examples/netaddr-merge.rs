extern crate netaddr2;
use netaddr2::NetAddr;

fn main() {
	let nets: &[NetAddr] = &[
		"192.0.0.0/24".parse().unwrap(),
		"192.0.1.0/24".parse().unwrap(),
	];

	let merged: NetAddr = nets[0].merge(&nets[1]).unwrap();
	assert_eq!(merged, "192.0.0.0/23".parse().unwrap());
}
