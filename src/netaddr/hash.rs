#[cfg(test)]
mod tests {
	use crate::NetAddr;
	use std::hash::Hash;

	#[test]
	fn hash_same() {
		use std::collections::hash_map::DefaultHasher;
		let mut hasher = DefaultHasher::new();
		assert_eq!(
			"192.0.2.26/29"
				.parse::<NetAddr>()
				.unwrap()
				.hash(&mut hasher),
			"192.0.2.26/29"
				.parse::<NetAddr>()
				.unwrap()
				.hash(&mut hasher)
		);
	}
}
