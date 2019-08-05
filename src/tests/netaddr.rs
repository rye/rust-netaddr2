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
}

mod netv6addr {
	use super::Netv6Addr;

	#[test]
	fn is_send() {
		fn assert_send<T: Send>() {}
		assert_send::<Netv6Addr>();
	}

	#[test]
	fn is_sync() {
		fn assert_sync<T: Sync>() {}
		assert_sync::<Netv6Addr>();
	}
}

mod broadcast;
mod cmp;
mod contains;
mod from;
mod merge;
mod parse;
