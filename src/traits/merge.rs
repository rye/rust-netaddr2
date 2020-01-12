/// Merge two items together
pub trait Merge {
	/// The type of the result of the merge
	type Output;

	/// Perform the merging operation
	fn merge(&self, other: &Self) -> Self::Output;
}
