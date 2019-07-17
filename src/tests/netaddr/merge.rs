use super::*;

#[test]
fn v4_adjacent_networks_correct() {
	let a: NetAddr = "10.0.0.0/24".parse().unwrap();
	let b: NetAddr = "10.0.1.0/24".parse().unwrap();

	assert_eq!(a.merge(&b), Some("10.0.0.0/23".parse().unwrap()));
}

#[test]
fn v4_adjacent_networks_reflexive() {
	let a: NetAddr = "10.0.0.0/24".parse().unwrap();
	let b: NetAddr = "10.0.1.0/24".parse().unwrap();

	assert_eq!(b.merge(&a), a.merge(&b));
}

#[test]
fn v4_nested_networks_takes_biggest() {
	let a: NetAddr = "10.0.0.0/24".parse().unwrap();
	let b: NetAddr = "10.0.0.0/23".parse().unwrap();

	assert_eq!(a.merge(&b), Some(b));
}

#[test]
fn v4_nested_networks_reflexive() {
	let a: NetAddr = "10.0.0.0/24".parse().unwrap();
	let b: NetAddr = "10.0.0.0/23".parse().unwrap();

	assert_eq!(a.merge(&b), b.merge(&a));
}

#[test]
fn v4_adjacent_but_not_mergable_none() {
	let a: NetAddr = "10.0.1.0/24".parse().unwrap();
	let b: NetAddr = "10.0.2.0/24".parse().unwrap();

	assert_eq!(a.merge(&b), None);
	assert_eq!(b.merge(&a), None);
	assert_eq!(a.merge(&b), b.merge(&a));
}

#[test]
#[should_panic]
fn v6_not_implemented() {
	let a: NetAddr = "2001:db8:dead:beef::/64".parse().unwrap();
	let b: NetAddr = "2001:db8:dead:beee::/64".parse().unwrap();

	assert_eq!(a.merge(&b), None);
	assert_eq!(b.merge(&a), None);
}
