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
		Self::Value::from_str(value).map_err(de::Error::custom)
	}
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for NetAddr {
	fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
		deserializer.deserialize_str(NetAddrVisitor)
	}
}
