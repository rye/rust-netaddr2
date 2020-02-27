/// Check containment of one object within another
pub trait Contains<T> {
	fn contains(&self, other: &T) -> bool;
}
