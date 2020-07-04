#[cfg(test)]
mod tests {
	use super::super::Netv4Addr;
	use std::hash::Hash;

	#[test]
	fn hash_same() {
		use std::collections::hash_map::DefaultHasher;
		use std::hash::Hasher;

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
