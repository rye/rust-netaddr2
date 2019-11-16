use super::NetAddr;

use serde::{Serialize, Serializer};

#[cfg(feature = "serde")]
impl Serialize for NetAddr {
	fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		serializer.serialize_str(&self.to_string())
	}
}

#[cfg(test)]
mod tests {
	use super::NetAddr;
	use serde_test::{assert_tokens, Token};

	mod v4 {
		use super::*;

		#[test]
		fn test_ser_cidr_localhost() {
			let netaddr: NetAddr = "127.0.0.1/8".parse().unwrap();

			assert_tokens(&netaddr, &[Token::Str("127.0.0.0/8")]);
		}

		#[test]
		fn test_ser_non_cidr_localhost() {
			let netaddr: NetAddr = "127.0.0.1/251.255.255.7".parse().unwrap();

			assert_tokens(&netaddr, &[Token::Str("123.0.0.1/251.255.255.7")]);
		}
	}

	mod v6 {
		use super::*;

		#[test]
		fn test_ser_cidr_localhost() {
			let netaddr: NetAddr = "ff02::1/60".parse().unwrap();

			assert_tokens(&netaddr, &[Token::Str("ff02::/60")]);
		}

		#[test]
		fn test_ser_non_cidr_localhost() {
			let netaddr: NetAddr = "ff02:dead:beef::1/ff02:eeee:eeee::1".parse().unwrap();

			assert_tokens(
				&netaddr,
				&[Token::Str("ff02:ceac:aeee::1/ff02:eeee:eeee::1")],
			);
		}
	}
}
