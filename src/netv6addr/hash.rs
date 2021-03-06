#[cfg(test)]
mod tests {
	use super::super::Netv6Addr;
	use std::hash::Hash;

	#[test]
	fn hash_same() {
		use std::collections::hash_map::DefaultHasher;
		use std::hash::Hasher;

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
