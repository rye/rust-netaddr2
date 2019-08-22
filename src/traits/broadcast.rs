pub trait Broadcast {
	type Output;

	fn broadcast(&self) -> Self::Output;
}
