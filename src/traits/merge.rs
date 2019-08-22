pub trait Merge {
	type Output;

	fn merge(&self, other: &Self) -> Self::Output;
}
