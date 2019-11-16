use super::NetAddr;

use serde::{Serialize, Serializer};

#[cfg(feature = "serde")]
impl Serialize for NetAddr {
	fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		serializer.serialize_str(&self.to_string())
	}
}
