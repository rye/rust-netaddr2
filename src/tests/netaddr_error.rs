use crate::*;

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
