use super::Netv6Addr;

use serde::{Serialize, Serializer};

#[cfg(feature = "serde")]
impl Serialize for Netv6Addr {
	fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		serializer.serialize_str(&self.to_string())
	}
}

#[cfg(test)]
mod tests {
	use super::Netv6Addr;
	use serde_test::{assert_tokens, Token};

	#[test]
	fn test_ser_cidr_localhost() {
		let netaddr: Netv6Addr = "ff02::1/60".parse().unwrap();

		assert_tokens(&netaddr, &[Token::Str("ff02::/60")]);
	}

	#[test]
	fn test_ser_non_cidr_localhost() {
		let netaddr: Netv6Addr = "ff02:dead:beef::1/ff02:eeee:eeee::1".parse().unwrap();

		assert_tokens(
			&netaddr,
			&[Token::Str("ff02:ceac:aeee::1/ff02:eeee:eeee::1")],
		);
	}
}
