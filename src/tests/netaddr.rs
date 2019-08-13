use crate::*;

use super::*;

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

#[test]
fn is_send() {
	fn assert_send<T: Send>() {}
	assert_send::<NetAddr>();
}

#[test]
fn is_sync() {
	fn assert_sync<T: Sync>() {}
	assert_sync::<NetAddr>();
}

#[test]
fn hash_same() {
	use std::collections::hash_map::DefaultHasher;
	use std::hash::Hash;
	let mut hasher = DefaultHasher::new();
	assert_eq!(
		"192.0.2.26/29"
			.parse::<NetAddr>()
			.unwrap()
			.hash(&mut hasher),
		"192.0.2.26/29"
			.parse::<NetAddr>()
			.unwrap()
			.hash(&mut hasher)
	);
}

mod netv4addr {
	use super::Netv4Addr;

	#[test]
	fn is_send() {
		fn assert_send<T: Send>() {}
		assert_send::<Netv4Addr>();
	}

	#[test]
	fn is_sync() {
		fn assert_sync<T: Sync>() {}
		assert_sync::<Netv4Addr>();
	}

	#[test]
	fn hash_same() {
		use std::collections::hash_map::DefaultHasher;
		use std::hash::Hash;
		let mut hasher = DefaultHasher::new();
		assert_eq!(
			"192.0.2.26/29"
				.parse::<Netv4Addr>()
				.unwrap()
				.hash(&mut hasher),
			"192.0.2.26/29"
				.parse::<Netv4Addr>()
				.unwrap()
				.hash(&mut hasher)
		);
	}

}

mod netv6addr {
	use super::Netv6Addr;

	#[test]
	fn is_send() {
;		fn assert_send<T: Send>() {}
		assert_send::<Netv6Addr>();
	}

	#[test]
	fn is_sync() {
		fn assert_sync<T: Sync>() {}
		assert_sync::<Netv6Addr>();
	}

	#[test]
	fn hash_same() {
		use std::collections::hash_map::DefaultHasher;
		use std::hash::Hash;
		let mut hasher = DefaultHasher::new();
		assert_eq!(
			"2001:db8:dead:beef::1325/64"
				.parse::<Netv6Addr>()
				.unwrap()
				.hash(&mut hasher),
			"2001:db8:dead:beef::1325/64"
				.parse::<Netv6Addr>()
				.unwrap()
				.hash(&mut hasher)
		);
	}

}

mod broadcast;
mod cmp;
mod contains;
mod from;
mod merge;
mod parse;
