use super::*;

#[test]
fn v4_adjacent_networks_correct() {
	let a: NetAddr = "10.0.0.0/24".parse().unwrap();
	let b: NetAddr = "10.0.1.0/24".parse().unwrap();

	assert_eq!(a.merge(&b), Some("10.0.0.0/23".parse().unwrap()));
}
