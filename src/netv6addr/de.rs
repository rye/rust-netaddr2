use serde::{de, Deserialize, Deserializer};

use super::Netv6Addr;

#[cfg(feature = "serde")]
struct Netv6AddrVisitor;

#[cfg(feature = "serde")]
impl<'de> de::Visitor<'de> for Netv6AddrVisitor {
	type Value = Netv6Addr;

	fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
		formatter.write_str("a valid cidr/extended network address")
	}

	fn visit_str<E: de::Error>(self, value: &str) -> Result<Self::Value, E> {
		use core::str::FromStr;
		Self::Value::from_str(value).map_err(de::Error::custom)
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
	use super::Netv6Addr;
	use serde_test::{assert_de_tokens, Token};

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
