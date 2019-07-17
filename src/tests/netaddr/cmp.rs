use super::*;

#[test]
fn v4_different_networks() {
	let a: NetAddr = "1.0.0.0/8".parse().unwrap();
	let b: NetAddr = "2.0.0.0/8".parse().unwrap();

	assert_eq!(a.partial_cmp(&b), Some(std::cmp::Ordering::Less))
}

#[test]
fn v4_different_netmasks() {
	let a: NetAddr = "1.0.0.0/7".parse().unwrap();
	let b: NetAddr = "1.0.0.0/8".parse().unwrap();

	assert_eq!(a.partial_cmp(&b), Some(std::cmp::Ordering::Less))
}

#[test]
fn v4_different() {
	let a: NetAddr = "1.0.0.0/8".parse().unwrap();
	let b: NetAddr = "0.0.0.0/24".parse().unwrap();

	assert_eq!(a.partial_cmp(&b), Some(std::cmp::Ordering::Greater))
}

#[test]
fn v4_equal() {
	let a: NetAddr = "1.0.0.0/8".parse().unwrap();
	let b: NetAddr = "1.0.0.0/8".parse().unwrap();

	assert_eq!(a.partial_cmp(&b), Some(std::cmp::Ordering::Equal))
}
