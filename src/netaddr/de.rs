use serde::{de, Deserialize, Deserializer};

use super::NetAddr;

#[cfg(feature = "serde")]
struct NetAddrVisitor;

#[cfg(feature = "serde")]
impl<'de> de::Visitor<'de> for NetAddrVisitor {
	type Value = NetAddr;

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
impl<'de> Deserialize<'de> for NetAddr {
	fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
		deserializer.deserialize_str(NetAddrVisitor)
	}
}

#[cfg(test)]
mod tests {
	use super::NetAddr;
	use serde_test::{assert_de_tokens, assert_de_tokens_error, Token};

	#[test]
	fn malformed_produces_correct_error() {
		assert_de_tokens_error::<NetAddr>(
			&[Token::Str("asdf")],
			"invalid value: string \"asdf\", expected a valid cidr/extended network address",
		)
	}

	mod v4 {
		use super::*;

		#[test]
		fn test_de_cidr_localhost() {
			let netaddr: NetAddr = "127.0.0.1/8".parse().unwrap();

			assert_de_tokens(&netaddr, &[Token::Str("127.0.0.0/8")]);
		}

		#[test]
		fn test_de_non_cidr_localhost() {
			let netaddr: NetAddr = "127.0.0.1/251.255.255.7".parse().unwrap();

			assert_de_tokens(&netaddr, &[Token::Str("123.0.0.1/251.255.255.7")]);
		}
	}

	mod v6 {
		use super::*;

		#[test]
		fn test_de_cidr_localhost() {
			let netaddr: NetAddr = "ff02::1/60".parse().unwrap();

			assert_de_tokens(&netaddr, &[Token::Str("ff02::/60")]);
		}

		#[test]
		fn test_de_non_cidr_localhost() {
			let netaddr: NetAddr = "ff02:dead:beef::1/ff02:eeee:eeee::1".parse().unwrap();

			assert_de_tokens(
				&netaddr,
				&[Token::Str("ff02:ceac:aeee::1/ff02:eeee:eeee::1")],
			);
		}
	}
}
