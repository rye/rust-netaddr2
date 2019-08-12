pub trait Contains {
	fn contains<T: Copy>(&self, other: &T) -> bool
	where
		Self: From<T>;
}
