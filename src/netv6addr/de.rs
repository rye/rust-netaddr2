use core::{
	fmt::{self, Formatter},
	str::FromStr,
};

#[cfg(feature = "serde")]
use serde::{
	de::{self, Error, Visitor},
	Deserialize, Deserializer,
};

use crate::netv6addr::Netv6Addr;

#[cfg(feature = "serde")]
struct Netv6AddrVisitor;

#[cfg(feature = "serde")]
impl<'de> Visitor<'de> for Netv6AddrVisitor {
	type Value = Netv6Addr;

	fn expecting(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
		formatter.write_str("a valid cidr/extended network address")
	}

	fn visit_str<E: Error>(self, value: &str) -> Result<Self::Value, E> {
		Self::Value::from_str(value)
			.map_err(|_| Error::invalid_value(de::Unexpected::Str(value), &self))
	}
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for Netv6Addr {
	fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
		deserializer.deserialize_str(Netv6AddrVisitor)
	}
}

#[cfg(test)]
mod tests {
	use serde_test::{assert_de_tokens, assert_de_tokens_error, Token};

	use crate::netv6addr::Netv6Addr;

	#[test]
	fn malformed_produces_correct_error() {
		assert_de_tokens_error::<Netv6Addr>(
			&[Token::Str("asdf")],
			"invalid value: string \"asdf\", expected a valid cidr/extended network address",
		);
	}

	#[test]
	fn test_de_cidr_localhost() {
		let netaddr: Netv6Addr = "ff02::1/60".parse().unwrap();

		assert_de_tokens(&netaddr, &[Token::Str("ff02::/60")]);
	}

	#[test]
	fn test_de_non_cidr_localhost() {
		let netaddr: Netv6Addr = "ff02:dead:beef::1/ff02:eeee:eeee::1".parse().unwrap();

		assert_de_tokens(
			&netaddr,
			&[Token::Str("ff02:ceac:aeee::1/ff02:eeee:eeee::1")],
		);
	}
}
