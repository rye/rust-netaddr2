#[cfg(test)]
mod tests {
	use crate::NetAddr;
	use std::hash::Hash;

	#[test]
	fn hash_same() {
		use std::collections::hash_map::DefaultHasher;
		use std::hash::Hasher;

		let actual = {
			let mut hasher = DefaultHasher::new();
			"192.0.2.26/29"
				.parse::<NetAddr>()
				.unwrap()
				.hash(&mut hasher);
			hasher.finish()
		};

		let expected = {
			let mut hasher = DefaultHasher::new();
			"192.0.2.26/29"
				.parse::<NetAddr>()
				.unwrap()
				.hash(&mut hasher);
			hasher.finish()
		};

		assert_eq!(actual, expected);
	}
}
