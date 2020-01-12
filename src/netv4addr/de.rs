use serde::{de, Deserialize, Deserializer};

use super::Netv4Addr;

#[cfg(feature = "serde")]
struct Netv4AddrVisitor;

#[cfg(feature = "serde")]
impl<'de> de::Visitor<'de> for Netv4AddrVisitor {
	type Value = Netv4Addr;

	fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
		formatter.write_str("a valid cidr/extended network address")
	}

	fn visit_str<E: de::Error>(self, value: &str) -> Result<Self::Value, E> {
		use core::str::FromStr;
		Self::Value::from_str(value)
			.map_err(|_| de::Error::invalid_value(de::Unexpected::Str(value), &self))
	}
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for Netv4Addr {
	fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
		deserializer.deserialize_str(Netv4AddrVisitor)
	}
}

#[cfg(test)]
mod tests {
	use super::Netv4Addr;
	use serde_test::{assert_de_tokens, assert_de_tokens_error, Token};

	#[test]
	fn malformed_produces_correct_error() {
		assert_de_tokens_error::<Netv4Addr>(
			&[Token::Str("asdf")],
			"invalid value: string \"asdf\", expected a valid cidr/extended network address",
		)
	}

	#[test]
	fn test_de_cidr_localhost() {
		let netaddr: Netv4Addr = "127.0.0.1/8".parse().unwrap();

		assert_de_tokens(&netaddr, &[Token::Str("127.0.0.0/8")]);
	}

	#[test]
	fn test_de_non_cidr_localhost() {
		let netaddr: Netv4Addr = "127.0.0.1/251.255.255.7".parse().unwrap();

		assert_de_tokens(&netaddr, &[Token::Str("123.0.0.1/251.255.255.7")]);
	}
}
