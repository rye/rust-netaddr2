use super::Netv4Addr;

use serde::{Serialize, Serializer};

#[cfg(feature = "serde")]
impl Serialize for Netv4Addr {
	fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		serializer.serialize_str(&self.to_string())
	}
}

#[cfg(test)]
mod tests {
	use super::Netv4Addr;
	use serde_test::{assert_tokens, Token};

	#[test]
	fn test_ser_cidr_localhost() {
		let netaddr: Netv4Addr = "127.0.0.1/8".parse().unwrap();

		assert_tokens(&netaddr, &[Token::Str("127.0.0.0/8")]);
	}

	#[test]
	fn test_ser_non_cidr_localhost() {
		let netaddr: Netv4Addr = "127.0.0.1/251.255.255.7".parse().unwrap();

		assert_tokens(&netaddr, &[Token::Str("123.0.0.1/251.255.255.7")]);
	}
}
