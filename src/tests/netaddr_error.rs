use crate::*;

#[test]
fn is_send() {
	fn assert_send<T: Send>() {}
	assert_send::<NetAddrError>();
}

#[test]
fn is_sync() {
	fn assert_sync<T: Sync>() {}
	assert_sync::<NetAddrError>();
}

#[test]
fn is_display() {
	use core::fmt::Display;
	fn assert_display<T: Display>() {}
	assert_display::<NetAddrError>();
}
