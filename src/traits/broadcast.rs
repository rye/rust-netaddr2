/// A trait encapsulating the operation of computing the broadcast
/// address for applicable networks.
pub trait Broadcast {
	type Output;

	fn broadcast(&self) -> Self::Output;
}
