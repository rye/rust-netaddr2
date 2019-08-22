#[cfg(test)]
mod tests {
	use super::super::Netv4Addr;
	use std::hash::Hash;

	#[test]
	fn hash_same() {
		use std::collections::hash_map::DefaultHasher;
		let mut hasher = DefaultHasher::new();
		assert_eq!(
			"192.0.2.26/29"
				.parse::<Netv4Addr>()
				.unwrap()
				.hash(&mut hasher),
			"192.0.2.26/29"
				.parse::<Netv4Addr>()
				.unwrap()
				.hash(&mut hasher)
		);
	}
}
