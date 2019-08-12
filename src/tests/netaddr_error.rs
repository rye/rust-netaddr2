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

#[cfg(test)]
mod parse_error {
	use super::NetAddrError;
	#[test]
	fn right_message() {
		let error: NetAddrError = NetAddrError::ParseError("INNER_TEXT".into());
		let result: &str = &format!("{}", error);
		assert_eq!(result, "unable to parse address: INNER_TEXT");
	}
}
