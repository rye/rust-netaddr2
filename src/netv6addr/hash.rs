#[cfg(test)]
mod tests {
	use super::super::Netv6Addr;
	use std::hash::Hash;

	#[test]
	fn hash_same() {
		use std::collections::hash_map::DefaultHasher;
		let mut hasher = DefaultHasher::new();
		assert_eq!(
			"2001:db8:dead:beef::1325/64"
				.parse::<Netv6Addr>()
				.unwrap()
				.hash(&mut hasher),
			"2001:db8:dead:beef::1325/64"
				.parse::<Netv6Addr>()
				.unwrap()
				.hash(&mut hasher)
		);
	}
}
