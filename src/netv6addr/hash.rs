#[cfg(test)]
mod tests {
	use std::{
		collections::hash_map::DefaultHasher,
		hash::{Hash, Hasher},
	};

	use crate::netv6addr::Netv6Addr;

	#[test]
	fn hash_same() {
		let actual = {
			let mut hasher = DefaultHasher::new();
			"2001:db8:dead:beef::1325/64"
				.parse::<Netv6Addr>()
				.unwrap()
				.hash(&mut hasher);
			hasher.finish()
		};

		let expected = {
			let mut hasher = DefaultHasher::new();
			"2001:db8:dead:beef::1325/64"
				.parse::<Netv6Addr>()
				.unwrap()
				.hash(&mut hasher);
			hasher.finish()
		};

		assert_eq!(actual, expected);
	}
}
