#[cfg(test)]
mod tests {
	use std::{
		collections::hash_map::DefaultHasher,
		hash::{Hash, Hasher},
	};

	use crate::netv4addr::Netv4Addr;

	#[test]
	fn hash_same() {
		let actual = {
			let mut hasher = DefaultHasher::new();
			"192.0.2.26/29"
				.parse::<Netv4Addr>()
				.unwrap()
				.hash(&mut hasher);
			hasher.finish()
		};

		let expected = {
			let mut hasher = DefaultHasher::new();
			"192.0.2.26/29"
				.parse::<Netv4Addr>()
				.unwrap()
				.hash(&mut hasher);
			hasher.finish()
		};

		assert_eq!(actual, expected);
	}
}
