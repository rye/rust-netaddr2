/// Check containment of one object within another
pub trait Contains {
	fn contains<T: Copy>(&self, other: &T) -> bool
	where
		Self: From<T>;
}
